import pygame
import numpy as np
import time
import os

# Imports end

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
    
    def __init__(self, width, height, x, y):
        pygame.Surface.__init__(self, (width, height))
        self.pos = np.array([x, y])
    
    def screen_pos(self, screen_pos):
        return (self.pos[0]-screen_pos[0], self.pos[1]-screen_pos[1])


class CollisionObject(GameObject):
    
    # Defaul place for collision rect is in the center of surface
    def __init__(self, width, height, x, y, collision_width, collision_height, offset_x, offset_y):
        GameObject.__init__(self, width, height, x, y)
        self.collision_rect = pygame.Rect(width/2 - collision_width/2 + offset_x, 
                                          height/2 - collision_height/2 + offset_y, 
                                          collision_width, 
                                          collision_height)
        self.fill((200, 0, 0))
        
    def colliding(col_obj):
        rect = self.collision_rect.move(self.pos[0], self.pos[1])
        obj_rect = col_obj.collision_rect.move(col_obj.pos[0], col_obj.pos[1])
        return rect.colliderect(obj_rect)

    # returns indicies that are colliding
    def collision_list(list):
        ind = []
        for i in range(len(list)):
            if colliding(list[i]):
                ind.append(i)
        return ind


class Player(CollisionObject):
    
    def __init__(self, width, height, x, y, collision_width, collision_height, offset_x, offset_y):
        CollisionObject.__init__(self, width, height, x, y, collision_width, collision_height, offset_x, offset_y)
        self.fill((0,0,0))

    def move(self, dt, dx, dy):
        self.pos = self.pos + np.array([dt*dx,dt*dy])


class Key():
    
    def __init__(self, action, key):
        self.action = action
        self.key = key


class Game():
    
    keys = {0 : Key("right", pygame.K_d),
            1 : Key("left", pygame.K_a),
            2 : Key("jump", pygame.K_SPACE)}
    
    input_arr = [False for k in range(len(keys))]
    
    
    t = time.clock()
    screen_pos = [0,0]
    game_objects = []
    player = Player(32, 32, 100, 200, 32, 32, 0, 0)
    groud = CollisionObject(1000, 20, 0, 600, 1000, 20, 0, 0)
    game_objects.append(groud)
    
    def __init__(self):
        pygame.init()
        pygame.display.set_mode((1280, 720))
        pygame.display.set_caption("untitled_project")

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
        input = self.input_arr
        for i in range(len(input)):
            if input[i]:
                if i == 0:
                    self.player.move(dt, 160, 0)
                if i == 1:
                    self.player.move(dt, -160, 0)
        
        for o in self.game_objects:
            screen.blit(o, o.screen_pos(self.screen_pos))
        
        screen.blit(self.player, self.player.screen_pos(self.screen_pos))

    def main(self):
        
        clock = pygame.time.Clock()
        screen = pygame.display.get_surface()
        
        background = pygame.Surface(screen.get_size()).convert()
        background.fill((255,255,255))
        
        while 1:
            screen.blit(background, (0,0))
            
            clock.tick(60)
            
            dt = time.clock() - self.t
            self.t = time.clock()
            
            self.input()
            self.update(dt, screen)
            
            pygame.display.update()
            
            print("time elapsed: " + str(int(self.t)))
            print("FPS: " + str(int(1/dt)))


if __name__ == "__main__":
    game = Game()
    game.main()

