import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-otp',
  templateUrl: './otp.component.html',
  styleUrl: './otp.component.css'
})
export class OTPComponent implements OnInit{
  ngOnInit(): void {
    console.log("inside otp component")
  }
}
