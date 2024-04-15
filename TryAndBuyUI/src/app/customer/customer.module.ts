import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { CustomerRoutingModule } from './customer-routing.module';
import { ShopComponent } from './shop/shop.component';
import { ProductDetailsComponent } from './product-details/product-details.component';
import { SearchComponent } from './search/search.component';
import { CartComponent } from './cart/cart.component';
import { OrdersComponent } from './orders/orders.component'
import { ReactiveFormsModule } from '@angular/forms';
import { PaymentComponent } from './payment/payment.component';


@NgModule({
  declarations: [
    ShopComponent,
    ProductDetailsComponent,
    SearchComponent,
    CartComponent,
    OrdersComponent,
    PaymentComponent
  ],
  imports: [
    CommonModule,
    CustomerRoutingModule,
    ReactiveFormsModule
  ]
})
export class CustomerModule { }
