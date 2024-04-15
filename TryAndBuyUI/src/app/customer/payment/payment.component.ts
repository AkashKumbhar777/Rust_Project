import { Component, OnInit } from '@angular/core';
import { Cart, PriceSummary } from '../../models/dataTypes';
import { ActivatedRoute, Route, Router } from '@angular/router';
import { ShopService } from '../../services/shop.service';
import { Products } from '../../models/product.models';
import { ProductsService } from '../../services/products.servic';

@Component({
  selector: 'app-payment',
  templateUrl: './payment.component.html',
  styleUrl: './payment.component.css'
})
export class PaymentComponent implements OnInit{
  public cart: Cart[] | undefined
  selectedProductId:string
  quantity:number
  totalProducts:Products[]
  selectedProduct:Products
  totalpayment:number=0;
  public priceSummary: PriceSummary = {
    price: 0,
    discount: 0,
    tax: 0,
    delivery: 0,
    total: 0
  }

  constructor(private router: Router, private shopService: ShopService, private prdtService: ProductsService, private route:ActivatedRoute){}

  ngOnInit(): void {
    // this.loadCardDetails()
    // this.selectedProductId=this.route.snapshot.params['_id'];
    // this.quantity=this.route.snapshot.params['qtn'];
    // console.log(this.route.snapshot.params)
    // this.selectedProduct=this.prdtService.getOneProduct(this.selectedProductId);
    // for(const product of this.totalProducts){
    //   this.totalpayment +=product.price * product.quantity;
    // }

    this.shopService.selectedProducts$.subscribe(products => {
      if (Array.isArray(products)) {
        this.totalProducts = products;
        console.log('Selected Products:', this.totalProducts);
      } else {
        console.error('Invalid type for products:', products);
      }
    });
  }


  // loadCardDetails(){
  //   this.selectedProduct=this.prdtService.getOneProduct(this.selectedProductId);
  //   this.shopService.getCart().subscribe((res)=>{
  //     this.cart = res.cart.products
  //     // console.log(this.cart);
  //     let price = 0
  //     res.cart.products.forEach((item: any)=>{
  //       if(item.quantity && item.price){
  //         price+= +item.price * +item.quantity 
  //       }
  //     })
  //     this.priceSummary.price = price
  //     this.priceSummary.tax = price/10
  //     this.priceSummary.delivery = 100
  //     this.priceSummary.total = price + price/10 + 100
  //     // console.log(this.priceSummary.total);
  //     if(!this.cart?.length){
  //       this.router.navigate(['/'])
  //     }else{
  //       this.shopService.getCartCount()
  //     }
  //   })
  // }

  redirectToGoogle() {
    window.location.href = 'https://buy.stripe.com/test_00g003cuJ9csarucMM';
  }
  

}
