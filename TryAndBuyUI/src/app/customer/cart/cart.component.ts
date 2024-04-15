import { Component, OnInit } from '@angular/core';
import { Products } from '../../models/product.models';
import { CartService } from '../../services/cart.service';
import { PriceSummary } from '../../models/dataTypes';
import { LocationStrategy } from '@angular/common';
import { Router } from '@angular/router';
import { ShopService } from '../../services/shop.service';

@Component({
  selector: 'app-cart',
  templateUrl: './cart.component.html',
  styleUrl: './cart.component.css'
})
export class CartComponent implements OnInit{
  selectedProducts:Products[]
  public removeCartLink:boolean=false;
  public priceSummary: PriceSummary = {
    price: 0,
    discount: 0,
    tax: 0,
    delivery: 0,
    total: 0
  }
  constructor(private cartSrv:CartService,private locationStrategy: LocationStrategy, private router:Router,private shopSrv:ShopService){}
  ngOnInit(): void {
    this.loadCart();
  
  }
  
  loadCart(){
    this.selectedProducts=this.cartSrv.retrieveFromLocal();
    console.log(this.selectedProducts);
   let price=0;
    this.selectedProducts.forEach((item : any)=>{
      price +=item.price;
    })

    this.priceSummary.price = price
      this.priceSummary.tax = price/10
      this.priceSummary.delivery = 100
      this.priceSummary.total = price + price/10 + 100
  }
  removeFromCart(productId: number){
    this.cartSrv.removeFromLocal(productId);
    
  }
  checkoutOrder() {
    this.shopSrv.setSelectedProducts(this.selectedProducts);
    this.router.navigate(['/payment']);
  }

}
