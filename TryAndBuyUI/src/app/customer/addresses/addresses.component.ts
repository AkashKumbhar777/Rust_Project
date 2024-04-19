import { Component, OnInit } from '@angular/core';
import { address } from '../../models/dataTypes';
import { AddressService } from '../../services/address.service';
import { FormBuilder } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-addresses',
  templateUrl: './addresses.component.html',
  styleUrl: './addresses.component.css'
})
export class AddressesComponent implements OnInit {
  AllAddresses: address[] = [];

  constructor(private adsService: AddressService, private fb: FormBuilder, private router: Router,) { }
  addressForm = this.fb.group({
    address_id: [0],
    user_id: [0],
    address_line1: [''],
    address_line2: [''],
    city: [''],
    add_state: [''],
    postal_code: [''],
    country: [''],
  })
  ngOnInit(): void {
    const userid = parseInt(sessionStorage.getItem('userId'));
    this.adsService.getAddress(userid).subscribe((res) => {
      if (Array.isArray(res)) {
        console.log(res);
        this.AllAddresses = res;
      }
    }, (err) => {
      console.log(err);
    })
  }

  removeadd(id: number) {
    this.adsService.deleteAddress(id).subscribe((res) => {
      if (res === null) {
        this.getTimeout('suc');
      }
    }, (err) => {
      console.log(err);
    })
  }
  getTimeout(val: string) {
    if (val === 'suc') {
      setTimeout(() => {
        window.location.reload();
      }, 500);
    } else {
      setTimeout(() => {
      }, 4000);
    }
  }
  onSubmit() {
    const userid = parseInt(sessionStorage.getItem('userId'));
    this.addressForm.value.user_id = userid;
    console.log(this.addressForm.value.user_id);
    let productData = this.addressForm.value as address;
    console.log(productData);
    this.adsService.createAddress(productData).subscribe((res) => {
      if (res) {
        console.log(res);
        this.router.navigate(['/user']);
      }
    }, (err) => {
      console.log(err);
    })
  }
}
