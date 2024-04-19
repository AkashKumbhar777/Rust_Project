import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { EventEmitter, Injectable } from "@angular/core";
import { Router } from "@angular/router";
import { BehaviorSubject, catchError, throwError } from "rxjs";
import { Login } from "../models/dataTypes";

@Injectable({
  providedIn: 'root'
})
export class SellerSignupService {
  public url = '';
  public signupMsg = new EventEmitter<boolean>(false);
  public isCustomerLoggedIn = new BehaviorSubject<boolean>(false);
  constructor(
    private http: HttpClient,
    private router: Router
  ) { }

  ngOnInit(): void {

  }
  errorHandler(error: HttpErrorResponse) {
    // console.log(error.error.message);  
    return throwError(error);
  }

  signupUser(userData: Login) {
    userData.user_role = 'admin';
    let userDataAdmin = userData;
    return this.http.post<Login>(`${this.url}auth/register`, userDataAdmin)
      .pipe(catchError(this.errorHandler))
  }

  loginUser(userData: Login) {

  }
}