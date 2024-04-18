import { Component, OnInit } from '@angular/core';
import { BuyCart, Cart, Login, Order, PriceSummary, address } from '../../models/dataTypes';
import { ActivatedRoute, Route, Router } from '@angular/router';
import { Products } from '../../models/product.models';
import { ProductsService } from '../../services/products.servic';
import { CartService } from '../../services/cart.service';
import { AddressService } from '../../services/address.service';
import { BuyCartService } from '../../services/buycart.service';
import { OrderService } from '../../services/order.service';
import { HttpClient } from '@angular/common/http';
import { WindowRef } from '../../window-ref';
import { PaymentService } from '../../services/payment.service';

@Component({
  selector: 'app-payment',
  templateUrl: './payment.component.html',
  styleUrl: './payment.component.css'
})
export class PaymentComponent implements OnInit {
  public cart: Cart[] | undefined
  public removeCartLink: boolean = false;
  selectedProductId: string
  productQuantity = 1;
  selectedAddressId: number
  quantity: number
  totalProducts: Products[]
  selectedProduct: Products
  totalpayment: number = 0;
  total: number;
  AllAddresses: address[] = [];
  selectedAddressIndex: number = -1; // Initially no address is selected
  showAddressDetails: boolean = false;
  user: Login;
  address: address[];
  private paymentService: PaymentService
  productId: number
  finalObj = {
    customerDetail: {
      name: "Tushar Pimpalshende",
      phone: "8421807963",
      email: "pingme288@gmail.com"
    },
    cardDetail: {
      cardNumber: "",
      cardExpDate: "",
      cardCvv: ""
    },
    addressDetail: {
      address: "123 Main St",
      landmark: "Near Monument",
      city: "Pune",
      state: "Maharashtra",
      pincode: "442401"
    }
  };
  constructor(private router: Router, private prdtService: ProductsService, private route: ActivatedRoute, private cartSrv: CartService, private adsService: AddressService, private buycartsrv: BuyCartService, private ordersrv: OrderService, private winRef: WindowRef, private http: HttpClient) { }

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const proId = parseInt(params['product_id']);
      console.log('Product ID:', proId);
      this.prdtService.getProduct(proId).subscribe((res) => {
        if (res) {
          console.log(res);
          this.selectedProduct = res;
          this.totalpayment = this.selectedProduct.price;
          this.productId = this.selectedProduct.product_id;
        }
      }, (err) => {
        console.log(err);
      }
      )
    });
    let id = parseInt(sessionStorage.getItem('userId'));
    this.http.get<Login>(`http://localhost:3000/user/${id}`)
      .subscribe(data => {
        console.log("", data);
        this.user = data;
      });

    this.http.get<address[]>(`http://localhost:3000/address/${id}`)
      .subscribe(data => {
        console.log("data =  ", data)
        this.address = data;
        console.log("Address  = ", this.address);
      });

  }

  removeFromAllCart(productId: number) {
    const userid = parseInt(sessionStorage.getItem('userId'));
    this.cartSrv.deleteTryCart(userid, productId).subscribe((res) => {
      console.log(res);
      if (res === null) {
        console.log("remove item");
        this.getTimeout('suc');
        this.router.navigate(['/cart']);
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

  redirectToGoogle(prod_id: number, totalpayment: number, productQuantity: number) {
    const userid = parseInt(sessionStorage.getItem('userId'));
    const cart: BuyCart =
    {
      buy_cart_id: 0,
      user_id: userid,
      product_id: prod_id,
      quantity: productQuantity,
      total_amount: totalpayment
    }

    this.buycartsrv.addToBuyCart(cart).subscribe((res) => {
      if (res) {
        console.log(res);
      }
    }, (err) => {
      console.log(err);
    })
    this.adsService.getAddress(userid).subscribe((res) => {
      if (Array.isArray(res)) {
        console.log(res);
        this.AllAddresses = res;
        this.showAddressDetails = !this.showAddressDetails;
      }
    }, (err) => {
      console.log(err);
    })
  }


  selectAddress(index: number) {
    this.selectedAddressIndex = index;
  }


  handleQuantity(val: string) {

    if (this.productQuantity < 20 && val === 'plus') {
      this.productQuantity += 1
      this.totalpayment = this.selectedProduct.price * this.productQuantity;
      this.total = this.totalpayment + 100;
    }
    if (this.productQuantity > 0 && val === 'min') {
      this.productQuantity -= 1
      this.totalpayment = this.selectedProduct.price * this.productQuantity;
      this.total = this.totalpayment + 100;
    }
  }

  orderNow(prod_id: number, totalpayment: number, addressId) {
    // this.paynow();
    const userid = parseInt(sessionStorage.getItem('userId'));
    const currentTime = new Date();
    currentTime.setDate(currentTime.getDate() + 7);
    const formattedTime = `${currentTime.getFullYear()}-${(currentTime.getMonth() + 1).toString().padStart(2, '0')}-${currentTime.getDate().toString().padStart(2, '0')} ${currentTime.getHours().toString().padStart(2, '0')}:${currentTime.getMinutes().toString().padStart(2, '0')}:${currentTime.getSeconds().toString().padStart(2, '0')}`;
    console.log(formattedTime);
    const addId = parseInt(addressId)
    console.log(addId);
    const order: Order = {
      order_id: 0,
      user_id: userid,
      address_id: addId,
      product_id: prod_id,
      total_amount: totalpayment,
      order_status: 'True',
      order_date: formattedTime
    }
    console.log(order);
    this.ordersrv.addToOrder(order).subscribe((res) => {
      if (res) {
        console.log(res);
      }
    }, (err) => {
      console.log(err);
    })
  }

  paynow() {

    let options: any = {
      "key": "rzp_test_y39gdD0Y9KbAMu",
      "amount": 100,
      "name": "Try And Buy",
      "description": "dummy data",
      "image": "",
      "modal": {
        "escape": false
      },
      "prefill": {
        "name": this.user.first_name,
        "contact": "7720888632",//this.user.phone,
        "email": this.user.email[0],
        "method": '',
        'card[number]': this.finalObj.cardDetail.cardNumber,
        'card[expiry]': this.finalObj.cardDetail.cardExpDate,
        'card[cvv]': this.finalObj.cardDetail.cardCvv
      },
      "notes": {
        "address": this.finalObj.addressDetail.address + ', ' + this.finalObj.addressDetail.landmark + ', ' + this.finalObj.addressDetail.city + ', ' + this.finalObj.addressDetail.state + '-' + this.finalObj.addressDetail.pincode
      },
      "theme": {
        "color": "rgb(255, 255, 255)",
      }
    };

    options.handler = (response) => {
      console.log("Response:", response);
      if (response) {

        console.log("Payment successful!");
        this.orderNow(this.productId, this.total, this.selectedAddressId);
        this.router.navigate(['/orders']);
      }
      this.handlePaymentResponse(response);
    };

    let rzp = new this.winRef.nativeWindow.Razorpay(options);
    rzp.open();
  }


  handlePaymentResponse(response: any) {
    this.paymentService.processPaymentResponse(response)
      .subscribe((data) => {
        console.log("inside handlePaymentResponse ")
        console.log("Payment processed successfully:", data);

      }, (error) => {
        console.error("Error processing payment:", error);
      });
  }
}
