import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Login } from "../models/dataTypes";
import { throwError,catchError, BehaviorSubject } from "rxjs";
import { EventEmitter, Injectable, OnInit } from "@angular/core";
import { Router } from "@angular/router";
@Injectable({
    providedIn: 'root'
  })
export class CustomerSignupService{
    public url='';

    user:Login;
    public signupMsg = new EventEmitter<boolean>(false);
    public isCustomerLoggedIn = new BehaviorSubject<boolean>(false);
    constructor(
                private http:HttpClient,
                private router:Router
    ){}

    
    errorHandler(error: HttpErrorResponse){
        // console.log(error.error.message);  
        return throwError(error);
      }
    signupUser(userData: Login){
        return this.http.post<Login>(`${this.url}auth/register`, userData)
        .pipe(catchError(this.errorHandler))
      }

    loginUser(userData:Login){
        console.log("inside login user")
        console.log(userData.username,userData.password)
        // this.http.post<Signup>(`${this.url}auth/login`, userData)
        // .pipe(catchError(this.errorHandler))
        // .subscribe((res)=>{
        //     if(res && res.accessToken && res._id){
        //         if(res.isAdmin===false){
        //             this.isCustomerLoggedIn.next(true)
        //             localStorage.setItem('customer', JSON.stringify({_id: res._id, accessToken: res.accessToken}))
        //             this.router.navigate(['/'])
        //             //add local cart here

        //         }else{
        //             this.signupMsg.emit(true)
        //         }
        //     }
        // },(err)=>{
        //     if(err){
        //         this.signupMsg.emit(true)
        //     }
        // })

        if(userData.username==='cst@gmail.com' && userData.password==='cst'){
            localStorage.setItem('customer',JSON.stringify({_id:1,accessToken:'A'}))
                    this.router.navigate(['/'])
                    console.log(localStorage)
                }
                else{
                    this.signupMsg.emit(true);
                }
    }

    //get user by userid
    getUserById(userId:string) {
      this.http.get<Login>(`url/${userId}`)
        .subscribe(
          (user: Login) => {
            this.user = user;
            console.log('User details:', this.user);
          },
          (error) => {
            console.error('Error fetching user:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }

    //update user by id
    updateUser(userIdToUpdate:string,updatedUser:Login) {
      this.http.put<Login>(`http://your-api-url.com/update_user/${userIdToUpdate}`, updatedUser)
        .subscribe(
          (updatedUser: Login) => {
            console.log('User updated successfully:', updatedUser);
            // Optionally, you can perform some action after successful update
          },
          (error) => {
            console.error('Error updating user:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }

    //delete user by id
    deleteUser(userIdToDelete:string) {
      this.http.delete(`url/delete_user/${userIdToDelete}`)
        .subscribe(
          () => {
            console.log('User deleted successfully');
            // Optionally, you can perform some action after successful deletion
          },
          (error) => {
            console.error('Error deleting user:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }
}