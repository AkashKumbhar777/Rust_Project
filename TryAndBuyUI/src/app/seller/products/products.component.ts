import { Component, OnInit } from '@angular/core';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';
import { Router } from '@angular/router';

@Component({
  selector: 'app-products',
  templateUrl: './products.component.html',
  styleUrl: './products.component.css'
})
export class ProductsComponent implements OnInit{
  products: Products[];
  deleteMsg:string;
  constructor(private productSrv:ProductsService,private router:Router){}
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
  onDelete(productId: number ) {
    // console.log(productId);
    productId && this.productSrv.deleteProduct(productId).subscribe(
      (res) => {
        if (res===null) {
          console.log(res);
          this.getAllProducts();
          this.deleteMsg = 'Products Have Been Deleted';
          this.getTimeout();
          setTimeout(() => {
            window.location.reload(); // Reload the page after a delay
          }, 1000);
        }
      },
      (err) => {
        if (err) {
          console.log(err);
          this.deleteMsg = err.statusText;
          this.getTimeout();
        }
      }
    );
  }

  getTimeout(){
    setTimeout(() => {
      
    }, 4000);
  }

}
