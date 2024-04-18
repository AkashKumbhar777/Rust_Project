import { Component, OnInit } from '@angular/core';
import { OrderService } from '../../services/order.service';
import { Order, address } from '../../models/dataTypes';
import { AddressService } from '../../services/address.service';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';
import { Router } from '@angular/router';

@Component({
  selector: 'app-order',
  templateUrl: './order.component.html',
  styleUrl: './order.component.css'
})
export class OrderComponent implements OnInit {
  ordes: Order[] = [];
  logedIn = true;
  constructor(private orderSrv: OrderService, private addressSrv: AddressService, private productSrv: ProductsService, private router: Router) { }
  ngOnInit(): void {

    const userid = parseInt(sessionStorage.getItem('userId'));
    if (userid) {
      this.logedIn = true;
    }
    this.orderSrv.getOrderbyId(userid).subscribe((res) => {
      if (Array.isArray(res)) {
        this.ordes = res;
        console.log(this.ordes);
      }

    }, (err) => {
      console.log(err);
    })

  }
  cancelOrder(id: number) {
    this.orderSrv.deleteOrder(id).subscribe((res) => {
      if (res === null) {
        console.log(res);
        this.getTimeout('suc')
        window.location.reload();
      }
    }, (err) => {
      console.log(err);
    })
  }

  getTimeout(val: string) {
    if (val === 'suc') {
      setTimeout(() => {
        this.router.navigate(['/cart']);
        window.location.reload();
      }, 500);
    } else {
      setTimeout(() => {
      }, 4000);
    }
  }
}
