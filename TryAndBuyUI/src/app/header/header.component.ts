import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { ProductsService } from '../services/products.servic';
import { Products } from '../models/product.models';

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrl: './header.component.css'
})
export class HeaderComponent implements OnInit {
  public menuType: string = "default";
  id = localStorage.getItem('customer');
  allProducts: Products[];
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private http: HttpClient,
    private productsrv: ProductsService
  ) {

  }
  ngOnInit(): void {
    this.route.queryParams.subscribe(params => {
      const code = params['code'];
      if (code) {
        this.handleResponse(code);
      }
    });
    this.router.events.subscribe((res: any) => {
      let sellerStore = localStorage.getItem('admin')
      let sellerData = sellerStore && JSON.parse(sellerStore)

      let customerStore = localStorage.getItem('customer')
      let customerData = customerStore && JSON.parse(customerStore)

      if (sellerData && res.url.includes('products')) {
        this.menuType = 'seller'

      } else if (customerData) {
        this.menuType = 'customer'

      } else {
        this.menuType = 'default'
      }

    })

    this.getAllProducts();
  }
  signInOrSignUp() {
    window.location.href = 'https://AKASHPK.b2clogin.com/AKASHPK.onmicrosoft.com/oauth2/v2.0/authorize?p=B2C_1_SignUpSignIn&client_id=1aaf2355-35a8-428c-9bb6-064079ee6c40&nonce=defaultNonce&redirect_uri=http%3A%2F%2Flocalhost%3A4200%2F&scope=openid%20https%3A%2F%2FAKASHPK.onmicrosoft.com%2Fapi%2FTry%26Buy.write%20https%3A%2F%2FAKASHPK.onmicrosoft.com%2Fapi%2FTry%26Buy.read&response_type=code&prompt=login';
  }

  handleResponse(code: string) {
    console.log('Handling response with code:', code);
    this.http.post<any>('http://localhost:3000/authenticate', { code })
      .subscribe(
        response => {
          console.log(response);
        },
        error => {
          console.error(error);
        }
      );
  }
  onCustomerLogout() {
    localStorage.removeItem('customer')
    this.router.navigate(['/'])
    // this.shopService.cartDataLength.emit([])
  }

  onSellerLogout() {
    sessionStorage.removeItem('userId');
    this.router.navigate(['/']);
  }

  getAllProducts() {
    this.productsrv.getProducts().subscribe((res) => {
      if (res && res.length) {
        this.allProducts = res
      }
    })
    let user = localStorage.getItem('customer')
    if (user) {
      // this.shopService.getCartCount()
    }
  }
}
