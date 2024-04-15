import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { EventEmitter, Injectable } from "@angular/core";
import { Router } from "@angular/router";
import { BehaviorSubject, catchError, throwError } from "rxjs";
import { Signup } from "../models/dataTypes";

@Injectable({
    providedIn: 'root'
  })
export class SellerSignupService{
    public url='';
    public signupMsg = new EventEmitter<boolean>(false);
    public isCustomerLoggedIn = new BehaviorSubject<boolean>(false);
    constructor(
                private http:HttpClient,
                private router:Router
    ){}

    ngOnInit(): void {
        
    }
    errorHandler(error: HttpErrorResponse){
        // console.log(error.error.message);  
        return throwError(error);
      }

    signupUser(userData:Signup){
        userData.isAdmin=true;
        let userDataAdmin=userData;
        return this.http.post<Signup>(`${this.url}auth/register`,userDataAdmin)
        .pipe(catchError(this.errorHandler))
    }

    loginUser(userData:Signup){
        // this.http.post<Signup>(`${this.url}auth/login`, userData)
        // .pipe(catchError(this.errorHandler))
        // .subscribe((res)=>{
        //     if(res && res._id && res.accessToken){
        //         if(res.isAdmin===true){
        //             localStorage.setItem('admin',JSON.stringify({_id:res._id,accessToken:res.accessToken}))
        //             this.router.navigate(['/products'])
        //         }
        //         else{
        //             this.signupMsg.emit(true);
        //         }
        //     }
        // },(err)=>{
        //     if(err){
        //         this.signupMsg.emit(true)
        //     }
        // })

        if(userData.email==='admin@gmail.com' && userData.password==='admin'){
            localStorage.setItem('admin',JSON.stringify({_id:1,accessToken:'Admin'}))
                    this.router.navigate(['/products'])
                    console.log(localStorage);
                }
                else{
                    this.signupMsg.emit(true);
                }
        
    }
}