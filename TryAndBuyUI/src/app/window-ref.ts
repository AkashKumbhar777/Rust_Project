// import { Injectable } from '@angular/core';

// function _window() : any {
//    // return the global native browser window object
//    return window;
// }

// @Injectable()
// export class WindowRef {
//    get nativeWindow() : any {
//       return _window();
//    }
// }
// import { WINDOW } from './window-token';
import { Injectable, InjectionToken, NgModule } from '@angular/core';
export const WINDOW = new InjectionToken<Window>('Window');

// In your AppModule or any other module where you want to provide the window object:



function _window() : any {
   // return the global native browser window object
   return window;
}

@Injectable()
@NgModule({
   providers: [
     { provide: WINDOW, useValue: window }
   ]
 })
export class WindowRef {
   get nativeWindow() : any {
      return _window();
   }
}

//   ngOnInit() {
//     // Fetch user information from the backend when the component initializes
//     this.getUserInfo();
//   }

//   getUserInfo() {
//     // Make an HTTP GET request to fetch user information
//     this.http.get<User>('http://localhost:3000/user/5') // Adjust URL according to your backend route
//       .subscribe(response => {
//         this.user = response;
//       });
//   }