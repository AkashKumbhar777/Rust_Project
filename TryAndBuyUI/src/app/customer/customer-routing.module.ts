import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ShopComponent } from './shop/shop.component';
import { ProductDetailsComponent } from './product-details/product-details.component';
import { SearchComponent } from './search/search.component';
import { CartComponent } from './cart/cart.component';
import { OrdersComponent } from './orders/orders.component';
import { PaymentComponent } from './payment/payment.component';


const routes: Routes = [
  {path: '', children:[
    {path: '', component: ShopComponent},
    {path: 'product-details/:_id', component: ProductDetailsComponent},
    {path: 'search/:params', component: SearchComponent},
    {path: 'cart', component: CartComponent},
    {path: 'orders', component: OrdersComponent},
    {path: 'payment', component: PaymentComponent}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class CustomerRoutingModule { }
