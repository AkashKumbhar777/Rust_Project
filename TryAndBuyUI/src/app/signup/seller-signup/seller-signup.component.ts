import { Component, OnInit } from '@angular/core';
import { FormBuilder, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { SellerSignupService } from '../../services/seller-signup.service';
import { HttpClient } from '@angular/common/http';
import { Signup } from '../../models/dataTypes';

@Component({
  selector: 'app-seller-signup',
  templateUrl: './seller-signup.component.html',
  styleUrl: './seller-signup.component.css'
})
export class SellerSignupComponent implements OnInit{
  signupMsg: string = '';
  constructor(private fb:FormBuilder, private signupService: SellerSignupService,private http:HttpClient ){}

  signupForm= this.fb.group({
    username: ['', [Validators.required]],
    email: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required]]
  })

  loginForm = this.fb.group({
    email: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required]]
  })
  ngOnInit(): void {
    
  }

  onSignup(){
    if (this.signupForm.valid) {
      // Perform signup action
    } else {
      this.signupMsg = 'Please fill in all fields correctly.';
    }
  }
  onLogin(){
    let userData = this.loginForm.value as Signup
    this.signupService.loginUser(userData)
    
    this.signupService.signupMsg.subscribe((res)=>{
      if(res){
        // console.log(res);
        this.signupMsg = "Please Enter Valid Credentails"
        this.loginForm.reset()
      }
      
    })
  }

}
