import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { CustomerSignupService } from '../../services/customer-signup.service';
import { Login } from '../../models/dataTypes';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-customer-signup',
  templateUrl: './customer-signup.component.html',
  styleUrls: ['./customer-signup.component.css']
})
export class CustomerSignupComponent implements OnInit {
 
  signupMsg: string = '';

  constructor(private fb: FormBuilder,private signupService: CustomerSignupService,private http:HttpClient) {}

  signupForm= this.fb.group({
    username: ['', [Validators.required]],
    email: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required]]
  })

  loginForm = this.fb.group({
    username: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required]]
  })

  ngOnInit(): void {
   
  }

  onSignup() {
    if (this.signupForm.valid) {
      // Perform signup action        
    } else {
      this.signupMsg = 'Please fill in all fields correctly.';
    }
  }

  onLogin() {
    let userData = this.loginForm.value as Login
    this.signupService.loginUser(userData);
    
    this.signupService.signupMsg.subscribe((res)=>{
      if(res){
        // console.log(res);
        this.signupMsg = "Please Enter Valid Credentails"
        this.loginForm.reset()
      }
      
    })
  }
}
