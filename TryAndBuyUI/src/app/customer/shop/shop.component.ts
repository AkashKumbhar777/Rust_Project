import { Component, OnInit } from '@angular/core';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';

@Component({
  selector: 'app-shop',
  templateUrl: './shop.component.html',
  styleUrl: './shop.component.css'
})
export class ShopComponent implements OnInit{
allProducts:Products[];

  constructor(private productsrv:ProductsService){}
  ngOnInit(): void {
    this.getAllProducts();
  }
  getAllProducts(){
    this.productsrv.getProducts().subscribe((res)=>{
      if(res && res.length){
        this.allProducts = res
      }
    })
    let user = localStorage.getItem('customer')
    if (user){
      // this.shopService.getCartCount()
    }
  }
}
