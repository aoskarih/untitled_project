import pygame
import numpy as np
import time
import os

# Imports end

palette = {"black"      : (0, 0, 0),
           "dark blue"  : (29, 43, 83),
           "purple"     : (126, 37, 83),
           "dark green" : (0, 135, 81),
           "brown"      : (171, 82, 54),
           "dark grey"  : (95, 87, 79),
           "grey"       : (194, 195, 199),
           "white"      : (255, 241, 232),
           "red"        : (255, 0, 77),
           "orange"     : (255, 163, 0),
           "yellow"     : (255, 236, 39),
           "green"      : (0, 228, 54),
           "blue"       : (41, 173, 255),
           "spale"      : (131, 118, 156),
           "pink"       : (255, 119, 168),
           "peach"      : (255, 204, 170)}

palette_values = [v for k, v in palette.items()]

# Actual code begins

def load_png(name):
    fullname = os.path.join('data', name)
    try:
        image = pygame.image.load(fullname)
        if image.get_alpha() is None:
            image = image.convert()
        else:
            image = image.convert_alpha()
    except:
        print("CWD: " + os.getcwd())
        print('Cannot load image: '+ fullname)
        raise SystemExit
    return image, image.get_rect()


class GameObject(pygame.Surface):
    
    type = "object"
    
    def __init__(self, width, height, x, y):
        pygame.Surface.__init__(self, (width, height))
        self.pos = np.array([x, y])
    
    def screen_pos(self, screen_pos):
        return (self.pos[0]-screen_pos[0], self.pos[1]-screen_pos[1])


class CollisionObject(GameObject):
    
    # Defaul place for collision rect is in the center of surface
    def __init__(self, width, height, x, y, 
                 collision_width, collision_height, 
                 offset_x, offset_y):
        
        GameObject.__init__(self, width, height, x, y)
        self.collision_rect = pygame.Rect(width/2 - collision_width/2 + offset_x, 
                                          height/2 - collision_height/2 + offset_y, 
                                          collision_width, 
                                          collision_height)
        self.type = "collision object"
        
    def colliding(self, col_obj):
        rect = self.collision_rect.move(self.pos[0], self.pos[1])
        obj_rect = col_obj.collision_rect.move(col_obj.pos[0], col_obj.pos[1])
        return rect.colliderect(obj_rect)

    # returns indicies that are colliding
    def collision_list(self, list):
        ind = []
        for i in range(len(list)):
            if self.colliding(list[i]):
                ind.append(i)
        return ind
    
    # returns direction 0:none, 1:up, 2:down, 3:left, 4:right
    def collision_direction(self, col_obj):
        precision = 100
        rect = self.collision_rect.move(self.pos[0], self.pos[1])
        obj_rect = col_obj.collision_rect.move(col_obj.pos[0], col_obj.pos[1])
        
        if obj_rect.collidepoint(rect.left, rect.centery):
            return 3
        elif obj_rect.collidepoint(rect.right, rect.centery):
            return 4
        else:
            for i in range(precision):
                rect_u = pygame.Rect(rect.centerx, rect.top, 
                                     i/precision * rect.width, rect.height/2)
                rect_d = pygame.Rect(rect.centerx, rect.centery, 
                                     i/precision * rect.width, rect.height/2)
                if rect_u.colliderect(obj_rect):
                    return 1
                elif rect_d.colliderect(obj_rect):
                    return 2
        return 0


class GroundBlock(CollisionObject):
    
    def __init__(self, x, y):
        CollisionObject.__init__(self, 32, 32, x, y, 32, 32, 0, 0)
        self.fill(palette["dark green"])


class Hook(CollisionObject):
    
    width = 8
    height = 8
    
    length = 0
    attached = False
    launched = False
    
    speed = 1500
    max_distance = 1000
    min_length = 50
    
    velocity = np.array([0, 0])
    
    def __init__(self, player):
        CollisionObject.__init__(self, self.width, self.height, player.pos[0], player.pos[1], self.width, self.height, 0, 0)
        self.type = "hook"
        self.player = player
        self.fill(palette["green"])

    def move(self, dt):
        self.pos = self.pos + dt*np.array(self.velocity)
    
    def launch(self, angle):
        vel_0 = [0, 0] #self.player.velocity
        self.pos = np.array([self.pos[0] + 16*np.cos(angle) + self.player.get_rect().width/2 - self.width/2, self.pos[1] + 16*np.sin(angle)])
        self.velocity = np.array([vel_0[0] + np.cos(angle) * self.speed, vel_0[1] + np.sin(angle) * self.speed])
        self.launched = True
    
    def attach(self, environment):
        l = self.collision_list(environment)
        if len(l) > 0:
            print(l)
            self.velocity = np.array([0, 0])
            self.length = self.distance_to_player()
            self.attached = True
        if self.distance_to_player() > self.max_distance:
            self.release()
    
    def distance_to_player(self):
        p1 = self.get_rect().center + self.pos
        p2 = self.player.get_rect().center + self.player.pos
        return np.sqrt((p1[0]-p2[0])**2+(p1[1]-p2[1])**2)
        
    def release(self):
        self.attached = False
        self.launched = False
        self.pos = self.player.pos

    def move_towards_point(self, d, point):
        p1 = point
        p2 = self.get_rect().center + self.pos
        r = np.array([p1[0]-p2[0], p1[1]-p2[1]])
        mag = np.sqrt(r.dot(r))
        r_unit = r/mag
        self.pos = self.pos + d*r_unit


class Player(CollisionObject):
    
    velocity = np.array([0, 0])
    grounded = False
    
    def __init__(self, width, height, x, y, collision_width, collision_height, offset_x, offset_y):
        CollisionObject.__init__(self, width, height, x, y, collision_width, collision_height, offset_x, offset_y)
        self.fill(palette["blue"])
        self.hook = Hook(self)
        self.type = "player"

    def move(self, dt):
        if not self.hook.launched:
            self.hook.velocity = self.velocity
            self.hook.move(dt)
        else:
            self.hook.move(dt)
        self.pos = self.pos + dt*np.array(self.velocity)
    
    def move_towards_point(self, d, point):
        p1 = point
        p2 = self.get_rect().center + self.pos
        r = np.array([p1[0]-p2[0], p1[1]-p2[1]])
        mag = np.sqrt(r.dot(r))
        r_unit = r/mag
        self.pos = self.pos + d*r_unit

    def hook_velocity_change(self):
        p1 = self.hook.get_rect().center + self.hook.pos
        p2 = self.get_rect().center + self.pos
        r = np.array([p1[0]-p2[0], p1[1]-p2[1]])
        mag = np.sqrt(r.dot(r))
        r_unit = r/mag
        self.velocity = self.velocity - (r_unit.dot(self.velocity))*r_unit
        print((r_unit.dot(self.velocity))*r_unit, r_unit)


class Key():
    
    def __init__(self, action, key):
        self.action = action
        self.key = key


class Game():
    
    keys = {0 : Key("right", pygame.K_d),
            1 : Key("left", pygame.K_a),
            2 : Key("jump", pygame.K_SPACE),
            3 : Key("hook launch", pygame.K_w),
            4 : Key("quit", pygame.K_ESCAPE),
            5 : Key("hook release", pygame.K_e)}
    
    input_arr = [False for k in range(len(keys))]
    
    # constants
    time_speed = 1
    g = 750
    ground_speed = 80
    air_speed = 3
    max_ground_speed = 300
    min_ground_speed = 50
    max_fall_speed = 1000
    ground_drag = 0.6
    ground_drag_acc = 0.2
    jump_start = 300
    jump_long = 15
    jump_time = 0.3
    hook_reel = 3

    screen_size = (0, 0)
    
    t = time.clock()
    fps_limit = 60
    time_since_grounded = 0
    
    screen_pos = [-100, -100]
    game_objects = []
    environment = []
    
    player = Player(32, 32, 100, 200, 32, 32, 0, 0)
    
    
    for i in range(100):
        o = GroundBlock(-500+40*i, 700)
        game_objects.append(o)
        environment.append(o)
    
    
    def __init__(self, screen_size=(1280, 720)):
        pygame.init()
        pygame.display.set_mode(screen_size)
        pygame.display.set_caption("untitled_project")
        self.screen_size = screen_size

    def input(self):
        
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                quit()
            if event.type == pygame.KEYDOWN:
                for i in range(len(self.keys)):
                    if event.key == self.keys[i].key:
                        self.input_arr[i] = True
            if event.type == pygame.KEYUP:
                for i in range(len(self.keys)):
                    if event.key == self.keys[i].key:
                        self.input_arr[i] = False

    def update(self, dt, screen):
        
        # gravity
        
        print(dt, self.g)
        print(dt*self.g)
        self.player.velocity = self.player.velocity + dt*np.array([0, self.g])
        
        
        input = self.input_arr
        for i in range(len(input)):
            if input[i]:
                if i == 0:
                    if self.player.grounded:
                        self.player.velocity += np.array([self.ground_speed, 0])
                    else:
                        self.player.velocity += np.array([self.air_speed, 0])
                if i == 1:
                    if self.player.grounded:
                        self.player.velocity += np.array([-self.ground_speed, 0])
                    else:
                        self.player.velocity += np.array([-self.air_speed, 0])
                if i == 2:
                    if self.player.grounded:
                        self.player.velocity += np.array([0, -self.jump_start])
                    if self.time_since_grounded < self.jump_time:
                        self.player.velocity += np.array([0, -self.jump_long])
                if i == 3:
                    if not self.player.hook.launched:
                        self.player.hook.launch(self.get_angle())
                    if self.player.hook.attached and self.player.hook.length > self.player.hook.min_length:
                        self.player.hook.length -= self.hook_reel
                if i == 4:
                    quit()
                if i == 5:
                    if True or self.player.hook.attached:
                        self.player.hook.release()
        
        self.interactions(dt)
        
        self.set_screen()
        
        for o in self.game_objects:
            screen.blit(o, o.screen_pos(self.screen_pos))
        
        if self.player.hook.launched:
            p1 = list(self.player.hook.screen_pos(self.screen_pos))
            p2 = list(self.player.screen_pos(self.screen_pos))
            p1[0] += 4
            p1[1] += 4
            p2[0] += 16
            p2[1] += 16
            pygame.draw.line(screen, palette["purple"], p1, p2, 1)
            screen.blit(self.player.hook, self.player.hook.screen_pos(self.screen_pos))
        screen.blit(self.player, self.player.screen_pos(self.screen_pos))

    def set_screen(self):
        self.screen_pos = self.player.pos - np.array(self.screen_size)/2


    def interactions(self, dt):
        
        # Player movement
        if self.player.grounded:
            if self.input_arr[0]:
                if self.player.velocity[0] > self.max_ground_speed:
                    self.player.velocity[0] = self.max_ground_speed
                elif self.player.velocity[0] < 0:
                    if np.abs(self.player.velocity[0]) < self.min_ground_speed:
                        self.player.velocity[0] = 0
                    else:
                        self.player.velocity[0] *= self.ground_drag_acc
            elif self.input_arr[1]:
                if self.player.velocity[0] < -self.max_ground_speed:
                    self.player.velocity[0] = -self.max_ground_speed
                elif self.player.velocity[0] > 0:
                    if np.abs(self.player.velocity[0]) < self.min_ground_speed:
                        self.player.velocity[0] = 0
                    else:
                        self.player.velocity[0] *= self.ground_drag_acc
            else:
                if np.abs(self.player.velocity[0]) < self.min_ground_speed:
                    self.player.velocity[0] = 0
                else:
                    self.player.velocity[0] *= self.ground_drag
        else:
            if self.player.velocity[1] > self.max_fall_speed:
                self.player.velocity[1] = self.max_fall_speed
        
        self.player.move(dt)
        
        self.player.grounded = False
        self.time_since_grounded += dt
                
        # Player collisions
        for o in self.game_objects:
            if o.type == "collision object":
                if self.player.colliding(o):
                    d = self.player.collision_direction(o)
                    # direction 0:none, 1:up, 2:down, 3:left, 4:right
                    if d == 1:
                        self.player.velocity *= np.array([1, 0])
                        self.player.pos[1] = o.pos[1]+o.get_height()
                    if d == 2:
                        self.player.velocity *= np.array([1, 0])
                        self.player.pos[1] = o.pos[1]-self.player.get_height()+1
                        self.player.grounded = True
                        self.time_since_grounded = 0
                    if d == 3:
                        self.player.velocity *= np.array([0, 1])
                        self.player.pos[0] = o.pos[0]+o.get_width()
                    if d == 4:
                        self.player.velocity *= np.array([0, 1])
                        self.player.pos[0] = o.pos[0]-self.player.get_width()        
        
        # Hook physics
        if self.player.hook.attached:
            d = self.player.hook.distance_to_player()
            l = self.player.hook.length
            if d > l:
                self.player.move_towards_point(d-l, self.player.hook.get_rect().center + self.player.hook.pos)
                self.player.hook_velocity_change()
        
        if self.player.hook.launched and not self.player.hook.attached:
            self.player.hook.attach(self.environment)
            

    def get_angle(self):
        p_rect = self.player.get_rect()
        p = np.array(self.player.screen_pos(self.screen_pos)) + [p_rect.width/2, p_rect.height/2]
        m = np.array(pygame.mouse.get_pos())
        x = m[0] - p[0]
        y = m[1] - p[1]
        return np.arctan2(y,x)
    
    def main(self):
        
        avg_fps = []
        avg_len = 60
        
        clock = pygame.time.Clock()
        screen = pygame.display.get_surface()
        
        background = pygame.Surface(screen.get_size()).convert()
        background.fill(palette["white"])
        
        while 1:
            screen.blit(background, (0,0))
            
            clock.tick(self.fps_limit)
            
            dt = time.clock() - self.t
            self.t = time.clock()
            
            self.input()
            self.update(dt*self.time_speed, screen)
            
            for i, c in enumerate(palette_values):
                pygame.draw.rect(screen, c, pygame.Rect(i*32, 0, 32, 32))
            
            pygame.display.update()
            
            os.system('clear')
            avg_fps.append(1/dt)
            if len(avg_fps) > avg_len:
                avg_fps = avg_fps[-avg_len:]
            print("time elapsed: " + str(int(self.t)))
            print("FPS: " + str(int(sum(avg_fps)/avg_len)))
            print("Player pos: " + str(np.int_(self.player.pos)))
            print("Player vel: " + str(np.int_(self.player.velocity)))
            print("Time since grounded: " + str(self.time_since_grounded))
            print("Mouse angle: " + str(self.get_angle()))
            
            print()


if __name__ == "__main__":
    game = Game()
    game.main()

