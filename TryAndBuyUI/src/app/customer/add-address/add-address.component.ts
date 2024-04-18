import { Component, OnInit } from '@angular/core';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';
import { FormBuilder } from '@angular/forms';
import { address } from '../../models/dataTypes';
import { AddressService } from '../../services/address.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-add-address',
  templateUrl: './add-address.component.html',
  styleUrl: './add-address.component.css'
})
export class AdddAddress implements OnInit {
  allProducts: Products[];
  addressForm = this.fb.group({
    address_id: [0],
    user_id: [0],
    address_line1: [''],
    address_line2: [''],
    city: [''],
    add_state: [''],
    postal_code: [''],
    country: ['']
  })

  constructor(private productsrv: ProductsService, private fb: FormBuilder, private addressSrv: AddressService, private router: Router) { }
  ngOnInit(): void {
  }
  onSubmit() {
    let addressdata = this.addressForm.value as address;
    addressdata.user_id = parseInt(sessionStorage.getItem('user_id'));
    this.addressSrv.createAddress(addressdata).subscribe((res) => {
      if (res) {
        console.log(res);
        this.router.navigate(['/address', addressdata.user_id]);
      }
    }, (err) => {
      console.log(err);
    })
  }
}
