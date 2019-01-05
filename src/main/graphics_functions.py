import pygame
import numpy as np


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


# x,y       start position
# angle     direction
# d         square size
# a, p, ph  amplitude, period, phase
# distance  how long the pattern is
# count     how many squares
# color     color of the squares
def sin_squares(screen, x, y, angle, d, a, p, ph, distance, count, color):
    
    coords = [[x + t*np.cos(angle) + a*np.sin((2*np.pi*t + ph)/p)*np.sin(angle),
              x + t*np.sin(angle) - a*np.sin((2*np.pi*t + ph)/p)*np.cos(angle)] for t in range(0, distance, distance//count)]
    for u, v in coords:
        pygame.draw.rect(screen, color, pygame.Rect(u-d/2, v-d/2, d, d), 3)





def test(screen_size=(1280, 720)):
    pygame.init()
    pygame.display.set_mode(screen_size)
    pygame.display.set_caption("untitled_project")
    
    screen = pygame.display.get_surface()
    background = pygame.Surface(screen.get_size()).convert()
    background.fill(palette["white"])
    ph = 0
    size = 10
    d = 0
    while 1:
        
        if size > 15:
            d = -d
        elif size < 5:
            d = -d
        size += d
        ph += 2
        
        screen.blit(background, (0,0))
        
        sin_squares(screen, 0, 0, np.pi/4, size, 30, 200, ph+np.pi*200, 1000, 40, palette["blue"])
        sin_squares(screen, 0, 0, np.pi/4, size, 30, 200, ph, 1000, 40, palette["blue"])
        
        pygame.display.update()
        
    
if __name__ == "__main__":
    test()
