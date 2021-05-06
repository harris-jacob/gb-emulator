#include <SDL2/SDL.h>
#include <stdio.h>
#include <SDL.h>
 
int main(int argc, char ** argv) {
    int quit = 0;
    SDL_Event event;
 
    SDL_Init(SDL_INIT_VIDEO);
 
    SDL_Window * window = SDL_CreateWindow("SDL2 Displaying Image",
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 200, 200, SDL_WINDOW_SHOWN);

    SDL_Renderer * renderer = SDL_CreateRenderer(window, -1, 0);

    int HEIGHT = 144;
    int WIDTH = 160;

    for (int i = 0; i < HEIGHT; i++) {
      for(int j = 0; j <WIDTH; j++) {
        uint8_t r = i;
        uint8_t g = j;
        uint8_t b = 25;

        SDL_SetRenderDrawColor(renderer, r, g, b, 255);
        SDL_RenderDrawPoint(renderer, j, i);
      }
    }

    SDL_RenderPresent(renderer);

    // run loop 
    while (!quit)
    {
        SDL_WaitEvent(&event);
 
        switch (event.type)
        {
        case SDL_QUIT:
            quit = 1;
            break;
        }
    }
 
    SDL_Quit();
 
    return 0;
}