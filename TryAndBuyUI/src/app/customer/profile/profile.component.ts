import { Component, OnInit } from '@angular/core';
import { CustomerSignupService } from '../../services/customer-signup.service';
import { Login, address } from '../../models/dataTypes';
import { Router } from '@angular/router';
import { AddressService } from '../../services/address.service';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [],
  templateUrl: './profile.component.html',
  styleUrl: './profile.component.css'
})
export class ProfileComponent implements OnInit {
  showAddressDetails: boolean = false;
  AllAddresses: address[] = [];
  public id: number = parseInt(sessionStorage.getItem('userId'));
  userDetails: Login
  constructor(private customersrv: CustomerSignupService, private router: Router, private adsService: AddressService) { }
  ngOnInit(): void {
    this.getUserById();
  }
  getUserById() {
    this.customersrv.getUserById(this.id).subscribe(
      (res) => {
        if (res) {
          // User found, assign user details
          this.userDetails = res;
          console.log(this.userDetails);
        } else {
          // Handle case when user is not found
          console.log('User not found');
        }
      },
      (error) => {
        // Handle error
        console.error('Error fetching user details:', error);
      }
    );
  }
  updateProfile() {
    this.router.navigate(['/user', this.userDetails.user_id]);
  }

  address() {
    this.router.navigate(['/user/address', this.userDetails.user_id]);

  }

}
