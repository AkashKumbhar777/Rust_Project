import { Component, OnInit } from '@angular/core';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';

@Component({
  selector: 'app-products',
  templateUrl: './products.component.html',
  styleUrl: './products.component.css'
})
export class ProductsComponent implements OnInit{
  products: Products[];
  deleteMsg:string;
  constructor(private productSrv:ProductsService){}
  ngOnInit(): void {
    this.getAllProducts();
  }
  getAllProducts(){
    this.productSrv.getProducts().subscribe((res)=>{
      if(res){
        this.products = res
        console.log(this.products);
      }
    })
  }
  onDelete(productId: number | undefined){
    console.log(productId);
    productId && this.productSrv.deleteProduct(productId).subscribe((res)=>{
      if(res){
        this.getAllProducts()
        this.deleteMsg = 'Products Has Been Deleted'
      }
      this.getTimeout()
      
    }, (err)=>{
      if(err){
        this.deleteMsg = err.statusText
      }
      this.getTimeout()
    })
  }

  getTimeout(){
    setTimeout(() => {
      this.deleteMsg = undefined
    }, 4000);
  }

}
