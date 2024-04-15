import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SignupRoutingModule } from './signup-routing.module';
import { CustomerSignupComponent } from './customer-signup/customer-signup.component';
import { SellerSignupComponent } from './seller-signup/seller-signup.component';
import { ReactiveFormsModule ,FormsModule} from '@angular/forms';
import { LoginComponent } from './login/login.component';


@NgModule({
  declarations: [
    CustomerSignupComponent,
    SellerSignupComponent,
    LoginComponent
  ],
  imports: [
    CommonModule,
    SignupRoutingModule,
    ReactiveFormsModule,
    FormsModule
  ]
})
export class SignupModule { }
