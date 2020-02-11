import pygame
import numpy as np
from PIL import Image
from scipy import special
import time

screen_size = (1280, 720)
tex_size = (200, 300)   # 320, 180

COLOR_GRAD = [
        12, 7, 135,
        16, 7, 136,
        19, 6, 137,
        22, 6, 138,
        24, 6, 140,
        27, 6, 141,
        29, 6, 142,
        31, 5, 143,
        33, 5, 144,
        35, 5, 145,
        38, 5, 146,
        40, 5, 146,
        42, 5, 147,
        43, 5, 148,
        45, 4, 149,
        47, 4, 150,
        49, 4, 151,
        51, 4, 151,
        53, 4, 152,
        54, 4, 153,
        56, 4, 154,
        58, 4, 154,
        60, 3, 155,
        61, 3, 156,
        63, 3, 156,
        65, 3, 157,
        66, 3, 158,
        68, 3, 158,
        70, 3, 159,
        71, 2, 160,
        73, 2, 160,
        75, 2, 161,
        76, 2, 161,
        78, 2, 162,
        80, 2, 162,
        81, 1, 163,
        83, 1, 163,
        84, 1, 164,
        86, 1, 164,
        88, 1, 165,
        89, 1, 165,
        91, 0, 165,
        92, 0, 166,
        94, 0, 166,
        95, 0, 166,
        97, 0, 167,
        99, 0, 167,
        100, 0, 167,
        102, 0, 167,
        103, 0, 168,
        105, 0, 168,
        106, 0, 168,
        108, 0, 168,
        110, 0, 168,
        111, 0, 168,
        113, 0, 168,
        114, 0, 169,
        116, 0, 169,
        117, 0, 169,
        119, 1, 168,
        120, 1, 168,
        122, 1, 168,
        123, 2, 168,
        125, 2, 168,
        126, 3, 168,
        128, 3, 168,
        129, 4, 167,
        131, 4, 167,
        132, 5, 167,
        134, 6, 167,
        135, 7, 166,
        136, 7, 166,
        138, 8, 166,
        139, 9, 165,
        141, 11, 165,
        142, 12, 164,
        144, 13, 164,
        145, 14, 163,
        146, 15, 163,
        148, 16, 162,
        149, 17, 161,
        150, 18, 161,
        152, 19, 160,
        153, 20, 160,
        155, 21, 159,
        156, 23, 158,
        157, 24, 157,
        158, 25, 157,
        160, 26, 156,
        161, 27, 155,
        162, 28, 154,
        164, 29, 154,
        165, 30, 153,
        166, 32, 152,
        167, 33, 151,
        169, 34, 150,
        170, 35, 149,
        171, 36, 149,
        172, 37, 148,
        173, 38, 147,
        175, 40, 146,
        176, 41, 145,
        177, 42, 144,
        178, 43, 143,
        179, 44, 142,
        180, 45, 141,
        181, 46, 140,
        183, 47, 139,
        184, 49, 138,
        185, 50, 137,
        186, 51, 137,
        187, 52, 136,
        188, 53, 135,
        189, 54, 134,
        190, 55, 133,
        191, 57, 132,
        192, 58, 131,
        193, 59, 130,
        194, 60, 129,
        195, 61, 128,
        196, 62, 127,
        197, 63, 126,
        198, 64, 125,
        199, 66, 124,
        200, 67, 123,
        201, 68, 122,
        202, 69, 122,
        203, 70, 121,
        204, 71, 120,
        205, 72, 119,
        206, 73, 118,
        207, 75, 117,
        208, 76, 116,
        208, 77, 115,
        209, 78, 114,
        210, 79, 113,
        211, 80, 112,
        212, 81, 112,
        213, 83, 111,
        214, 84, 110,
        215, 85, 109,
        215, 86, 108,
        216, 87, 107,
        217, 88, 106,
        218, 89, 105,
        219, 91, 105,
        220, 92, 104,
        220, 93, 103,
        221, 94, 102,
        222, 95, 101,
        223, 96, 100,
        224, 98, 99,
        224, 99, 98,
        225, 100, 98,
        226, 101, 97,
        227, 102, 96,
        227, 104, 95,
        228, 105, 94,
        229, 106, 93,
        230, 107, 92,
        230, 108, 92,
        231, 110, 91,
        232, 111, 90,
        232, 112, 89,
        233, 113, 88,
        234, 114, 87,
        235, 116, 86,
        235, 117, 86,
        236, 118, 85,
        237, 119, 84,
        237, 121, 83,
        238, 122, 82,
        238, 123, 81,
        239, 124, 80,
        240, 126, 80,
        240, 127, 79,
        241, 128, 78,
        241, 129, 77,
        242, 131, 76,
        242, 132, 75,
        243, 133, 74,
        244, 135, 73,
        244, 136, 73,
        245, 137, 72,
        245, 139, 71,
        246, 140, 70,
        246, 141, 69,
        247, 143, 68,
        247, 144, 67,
        247, 145, 67,
        248, 147, 66,
        248, 148, 65,
        249, 149, 64,
        249, 151, 63,
        249, 152, 62,
        250, 154, 61,
        250, 155, 60,
        251, 156, 60,
        251, 158, 59,
        251, 159, 58,
        251, 161, 57,
        252, 162, 56,
        252, 164, 55,
        252, 165, 54,
        252, 166, 54,
        253, 168, 53,
        253, 169, 52,
        253, 171, 51,
        253, 172, 50,
        253, 174, 49,
        254, 175, 49,
        254, 177, 48,
        254, 178, 47,
        254, 180, 46,
        254, 181, 46,
        254, 183, 45,
        254, 185, 44,
        254, 186, 43,
        254, 188, 43,
        254, 189, 42,
        254, 191, 41,
        254, 192, 41,
        254, 194, 40,
        254, 195, 40,
        254, 197, 39,
        254, 199, 39,
        253, 200, 38,
        253, 202, 38,
        253, 203, 37,
        253, 205, 37,
        253, 207, 37,
        252, 208, 36,
        252, 210, 36,
        252, 212, 36,
        251, 213, 36,
        251, 215, 36,
        251, 217, 36,
        250, 218, 36,
        250, 220, 36,
        249, 222, 36,
        249, 223, 36,
        248, 225, 37,
        248, 227, 37,
        247, 229, 37,
        247, 230, 37,
        246, 232, 38,
        246, 234, 38,
        245, 235, 38,
        244, 237, 39,
        244, 239, 39,
        243, 241, 39,
        242, 242, 38,
        242, 244, 38,
        241, 246, 37,
        241, 247, 36,
        240, 249, 33
]

c = 0.1
g = 1
v = 1
scale = 0.2

def func(x0, y0, t):
    x1 = -160 + x0
    y1 = -90 + y0
    r = np.sqrt(x1*x1 + y1*y1) - c * t
    s = 10.0
    if abs(r) > 3*s:
        return 0
    return s/(abs(0.1*r*r)/s + 1) * np.exp(-r*r*(1./500))

def z(rho, phi):

    f = lambda t, p : - np.cos(t + p) / np.cos(t)**2

    thp = .5 * (np.arcsin(3*np.sin(phi), dtype=complex) - phi)
    thm = .5 * (np.pi - phi - np.arcsin(3*np.sin(phi), dtype=complex))

#    print("thp: " + str(thp))
#    print("thm: " + str(thm))

    cosp = np.cos(thp)
    secp = 1./cosp
    
    cosm = np.cos(thm)
    secm = 1./cosm

    dfp = - secp**2 * (np.cos(phi + thp) * (4 * np.tan(thp)**2 + 2*secp**2 - 1) - 4*np.tan(thp)*np.sin(phi + thp))
    dfm = - secm**2 * (np.cos(phi + thm) * (4 * np.tan(thm)**2 + 2*secm**2 - 1) - 4*np.tan(thm)*np.sin(phi + thm))

#    print("dfp: " + str(dfp))
#    print("dfm: " + str(dfm))

    fti = 1./2 * (f(thp, phi) + f(thm, phi))
    det = np.power((3./4 * (f(thm, phi) - f(thp, phi))), 2./3, dtype=complex)

#    print("fti: " + str(fti))
#    print("det: " + str(det))

    u = np.sqrt(np.sqrt(det)/2) * (1./np.sqrt(dfp) + 1./np.sqrt(-dfm))
    v = np.sqrt(1./(2*np.sqrt(det))) * (1./np.sqrt(dfp) - 1./np.sqrt(-dfm))

#    print("u: " + str(u))
#    print("v: " + str(v))

    ai, aip, _, _ = special.airy([- np.power(rho, 2./3) * det])

#    print("ai: " + str(ai[0]))
#    print("aip: " + str(aip[0]))

    val = 2*np.pi * (np.power(rho, -1./3) * u * np.cos(rho * fti) * ai[0] + np.power(rho, -2./3) * v * np.sin(rho * fti) * aip[0])

    return val

def wake(x, y):
    x -= tex_size[0]/2
    x *= scale
    y *= scale
    rho = g*np.sqrt(x*x + y*y)/v**2
    phi = np.arccos(float(y)/rho)

    s = 50
#    print("rho: " + str(rho))
#    print("phi: " + str(phi))
    val = z(rho, phi)
#    print("Value: " + str(val))
    if abs(phi) > 0.3398369:
        return s * (1 - val)
    else:
        return s * (1 + val)

def color_from_value(val):
    i = 0
    if val > 0.0 and val < 100.0:
        i = int(256.0*(val/100.0))
    c = (COLOR_GRAD[i*3 + 0], COLOR_GRAD[i*3 + 1], COLOR_GRAD[i*3 + 2]) # r, g, b
    return c


def main():
    
    screen = pygame.display.get_surface()
    background = pygame.Surface(tex_size)
    background.fill(color_from_value(0))
    px = pygame.PixelArray(background)

    f = 1000

    sep = 20
    n = 10
    
    while f < 10000:
        for x in range(320):
            for y in range(180):
                val = 0
                for i in range(n):
                    if i*100 < f: 
                        val += func(x - n/2 * sep + i*sep, y, f - i*100)
                px[x, y] = color_from_value(val)
        print(f)
        s = px.make_surface()
        screen.blit(pygame.transform.scale(s, (1280, 720)), (0,0))
        f += 1
        pygame.display.update()

def wake_pic():
    px = []
    value = []
    for y in range(tex_size[1]):
        row = []
        values = []
        for x in range(tex_size[0]):
            val = wake(x+0.5, y+0.5)
            #print(val)
            c = color_from_value(abs(val.real))
            values.append(abs(val.real))
            row.append(c)
        print("Row " + str(y) + " ready!")
        value.append(values)
        px.append(row)
    
    pixels = np.array(px, dtype=np.uint8)
    new_image = Image.fromarray(pixels)
    new_image.save("kelvin_wake.png")

    array_to_sprite(value, [19, 0], "test.s")


def array_to_sprite(array, origin, filename, swap_axis=False):
    x0 = -origin[0]
    f = open(filename, "w")
    for y in range(tex_size[1]):
        y0 = y - origin[1]
        line = "> " + str(x0) + " " + str(y0) + " > "
        for x in range(tex_size[0]):
            v = array[y][x]
            i = 4096 + int(256.0*(v/100.0))
            line += hex(i)[-2:] + " "
        line += "\n"
        f.write(line)
    

if __name__ == "__main__":    
#    pygame.init()
#    pygame.display.set_mode(screen_size)
#    main()
    wake_pic()
    