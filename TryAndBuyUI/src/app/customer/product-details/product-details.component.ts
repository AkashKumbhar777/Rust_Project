import { Component, OnInit } from '@angular/core';
import { Products } from '../../models/product.models';
import { ProductsService } from '../../services/products.servic';
import { ActivatedRoute, Router } from '@angular/router';
import { CartService } from '../../services/cart.service';
import { BuyCart, Cart } from '../../models/dataTypes';

@Component({
  selector: 'app-product-details',
  templateUrl: './product-details.component.html',
  styleUrl: './product-details.component.css'
})
export class ProductDetailsComponent implements OnInit {
  public removeCartLink: boolean = false;
  public productQantity: number = 1;
  id: number;
  public productDetails: Products | undefined;
  prdt: Products



  constructor(private productSrv: ProductsService,
    private route: ActivatedRoute,
    private cartSrv: CartService,
    private router: Router,
    private cartService: CartService
  ) { }
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.id = parseInt(params['_id']); // Assuming the parameter name is 'id'
      this.getProductDetails();
    });
  }
  getProductDetails() {
    this.productSrv.getProduct(this.id).subscribe((res) => {
      if (res) {
        this.productDetails = res;
      }
    })

  }
  addToCart(id: number) {
    const currentTime = new Date();
    const formattedTime = `${currentTime.getFullYear()}-${(currentTime.getMonth() + 1).toString().padStart(2, '0')}-${currentTime.getDate().toString().padStart(2, '0')} ${currentTime.getHours().toString().padStart(2, '0')}:${currentTime.getMinutes().toString().padStart(2, '0')}:${currentTime.getSeconds().toString().padStart(2, '0')}`;
    console.log("Current time:", formattedTime);
    console.log("userid", sessionStorage.getItem('userId'));
    // Initialize data object
    let data: Cart = {
      try_cart_id: 0,
      added_at: formattedTime,
      user_id: parseInt(sessionStorage.getItem('userId')),
      product_id: id
    };
    console.log(data);

    this.cartService.addToCart(data).subscribe(
      (res) => {
        if (res) {
          console.log(res);
        }
      },
      (err) => {
        console.log(err);
      }
    );
  }

  
}
