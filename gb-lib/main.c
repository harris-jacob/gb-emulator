#include <SDL2/SDL.h>
#include <stdio.h>

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

int main(int argc, char* args[]) {
  SDL_Window* window = NULL;
  SDL_Renderer* renderer = NULL;
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "could not initialize sdl2: %s\n", SDL_GetError());
    return 1;
  }
  window = SDL_CreateWindow(
			    "tile_map",
			    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
			    SCREEN_WIDTH, SCREEN_HEIGHT,
			    SDL_WINDOW_SHOWN
			    );
  if (window == NULL) {
    fprintf(stderr, "could not create window: %s\n", SDL_GetError());
    return 1;
  }

  SDL_CreateWindowAndRenderer(160, 144, SDL_WINDOW_RESIZABLE, &window, &renderer);


  for (int i=0; i<= 160; i++) {
    for(int j=0; j<= 144; j++) {
      SDL_RenderDrawPoint(renderer, i, j);
      if(i%10 == 0) {
        SDL_SetRenderDrawColor(renderer, 255 - i, 255 - i, 255 - i, 1);
      }
      SDL_UpdateWindowSurface(window);
    }
  }

  SDL_Delay(10000);
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);

  SDL_Quit();

  return 0;
}