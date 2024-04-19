import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { CustomerSignupService } from '../../services/customer-signup.service';
import { Login } from '../../models/dataTypes';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

@Component({
  selector: 'app-customer-signup',
  templateUrl: './customer-signup.component.html',
  styleUrls: ['./customer-signup.component.css']
})
export class CustomerSignupComponent implements OnInit {
 
  signupMsg: string = '';
  userData: any;

  constructor(private fb: FormBuilder,private signupService: CustomerSignupService,private http:HttpClient,private router:Router) {}

  signupForm= this.fb.group({
    username: ['', [Validators.required]],
    email: ['', [Validators.required, Validators.email]]
  })

  loginForm = this.fb.group({
    email: ['', [Validators.required, Validators.email]],
    password:['',[Validators.required]]
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


  onLogin() : void {
    const email=this.loginForm.get('email').value;
    const password=this.loginForm.get('password').value;
    console.log(email);
    this.signupService.loginUser(email)
      .subscribe(
        
        (data) => {
          console.log(data);
          this.userData = data;
          console.log('User data:', this.userData);
          if(this.userData.user_role==='user'){
            sessionStorage.setItem('userId', this.userData.user_id);
            this.router.navigate(['/']);
            console.log(localStorage.getItem('user'));
            

          }
          else if(this.userData.user_role==='admin'){
            // this.router.navigate(['/products']);
            sessionStorage.setItem('userId', this.userData.user_id);
            this.router.navigate(['/products']);
            console.log(localStorage.getItem('admin'));
          }
          else{
            
          }
        },
        (error) => {
          console.error('Error:', error);
        }
      );
  }
}
