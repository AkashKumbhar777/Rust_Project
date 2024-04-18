import { AfterViewInit, Component, OnInit } from '@angular/core';
import { Products } from '../../models/product.models';
import { CartService } from '../../services/cart.service';
import { BuyCart, Cart, PriceSummary } from '../../models/dataTypes';
import { LocationStrategy } from '@angular/common';
import { Router } from '@angular/router';
import { ProductsService } from '../../services/products.servic';
import { BuyCartService } from '../../services/buycart.service';

@Component({
  selector: 'app-cart',
  templateUrl: './cart.component.html',
  styleUrl: './cart.component.css'
})
export class CartComponent implements OnInit {
  allCart: Cart[]
  selectedProduct: Products[] = [];
  public removeCartLink: boolean = false;
  private isFirstLoad = true;
  productQuantity = 1;
  public priceSummary: PriceSummary = {
    price: 0,
    discount: 0,
    tax: 0,
    delivery: 0,
    total: 0
  }
  constructor(private cartSrv: CartService, private locationStrategy: LocationStrategy, private router: Router, private productSrv: ProductsService, private buycartSrv: BuyCartService) { }
  ngOnInit(): void {
    this.getCartByUserId();

  }

  getCartByUserId() {

    const userId = parseInt(sessionStorage.getItem('userId'));
    this.cartSrv.getTryCartByUserId(userId).subscribe((res) => {
      if (Array.isArray(res)) {

        this.allCart = res;
        console.log(this.allCart);

        // console.log(this.allCart.product_id);
        this.allCart.forEach((product) => {
          console.log(product.product_id);
          this.productSrv.getProduct(product.product_id).subscribe((productRes) => {
            if (productRes) {
              this.selectedProduct.push(productRes); // Add productRes to selectedProducts array
              console.log(this.selectedProduct);
            }
          });

        })
      }

    }, (err) => {
      console.log(err);
    })

  }

  removeFromCart(productId: number) {
    const userid = parseInt(sessionStorage.getItem('userId'));
    this.cartSrv.deleteTryCart(userid, productId).subscribe((res) => {
      console.log(res);
      if (res === null) {
        console.log("remove item");
        this.getTimeout('suc');
      }
    }, (err) => {
      console.log(err);
      this.getTimeout('err')
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
  handleQuantity(val: string) {
    if (this.productQuantity < 20 && val === 'plus') {
      this.productQuantity += 1
    }
    if (this.productQuantity > 0 && val === 'min') {
      this.productQuantity -= 1
    }
  }

  addToBuyCart(proId: number) {


    console.log("inside buycart");
    this.router.navigate(['/buycart', proId])

  }

}
