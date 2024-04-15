import { Component, OnInit } from '@angular/core';
import { Products } from '../../models/product.models';
import { ProductsService } from '../../services/products.servic';
import { ActivatedRoute } from '@angular/router';
import { CartService } from '../../services/cart.service';

@Component({
  selector: 'app-product-details',
  templateUrl: './product-details.component.html',
  styleUrl: './product-details.component.css'
})
export class ProductDetailsComponent implements OnInit{
  public removeCartLink:boolean=false;
  public productQantity:number=1;

  public productDetails: Products | undefined;
  prdt:Products
  constructor(private productSrv:ProductsService,
              private route:ActivatedRoute ,
              private cartSrv:CartService     
    ){}
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const id = params['product_id']; // Assuming the parameter name is 'id'
      // this.productDetails=this.productSrv.getOneProduct(id);
    });
  }
  handleQuantity(vari:string){
    if(this.productQantity<20 && vari==='plus'){
      this.productQantity +=1;
    }
    if(this.productQantity>0 && vari==='min'){
      this.productQantity -=1;
    }
  }

  addToCart(productDetails:Products){
    console.log("inside add cart in pdt-details comp");
    console.log(productDetails);
    console.log(localStorage.getItem);
    this.cartSrv.addToLocal(productDetails);
  }  
  redirectToGoogle() {
    window.location.href = 'https://buy.stripe.com/test_00g003cuJ9csarucMM';
  }
}
