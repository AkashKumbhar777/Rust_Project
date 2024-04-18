import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Login } from "../models/dataTypes";
import { throwError, catchError, BehaviorSubject } from "rxjs";
import { EventEmitter, Injectable, OnInit } from "@angular/core";
import { Router } from "@angular/router";
@Injectable({
  providedIn: 'root'
})
export class CustomerSignupService {
  public url = 'http://localhost:3000/';

  user: Login;
  public signupMsg = new EventEmitter<boolean>(false);
  public isCustomerLoggedIn = new BehaviorSubject<boolean>(false);
  constructor(
    private http: HttpClient,
    private router: Router
  ) { }


  errorHandler(error: HttpErrorResponse) {
    return throwError(error);
  }
  signupUser(userData: Login) {
    return this.http.post<Login>(`${this.url}auth/register`, userData)
      .pipe(catchError(this.errorHandler))
  }

  //geuser by email
  loginUser(email: string) {
    console.log("inside login user")
    return this.http.get<Login>(`${this.url}userbyemail/${email}`)
      .pipe(catchError(this.errorHandler));
  }

  //get user by userid
  getUserById(userId: number) {
    return this.http.get<Login>(`${this.url}user/${userId}`)
      .pipe(catchError(this.errorHandler))
  }

  //update user by id
  updateUser(updatedUser: Login) {
    return this.http.put<Login>(`${this.url}user/update/${updatedUser.user_id}`, updatedUser)
      .pipe(catchError(this.errorHandler))
  }

  //delete user by id
  deleteUser(userIdToDelete: string) {
    return this.http.delete(`${this.url}user/delete/${userIdToDelete}`)
      .pipe(catchError(this.errorHandler))
  }

  //get all user
  getAllUsers() {
    return this.http.get(`${this.url}users`)
      .pipe(catchError(this.errorHandler))
  }
}