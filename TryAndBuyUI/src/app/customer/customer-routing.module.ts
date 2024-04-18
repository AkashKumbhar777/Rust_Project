import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ShopComponent } from './shop/shop.component';
import { ProductDetailsComponent } from './product-details/product-details.component';
import { CartComponent } from './cart/cart.component';
import { PaymentComponent } from './payment/payment.component';
import { ProfileComponent } from './profile/profile.component';
import { OrderComponent } from './order/order.component';
import { AddressesComponent } from './addresses/addresses.component';
import { HeaderComponent } from '../header/header.component';


const routes: Routes = [
  {path: '', children:[
    {path: '', component: HeaderComponent},
    {path: 'shop', component: ShopComponent},
    {path: 'product-details/:_id', component: ProductDetailsComponent},
    {path: 'cart', component: CartComponent},
    {path: 'orders', component: OrderComponent},
    {path: 'buycart/:product_id', component: PaymentComponent}
  ]},
  {path:'user',children:[
    {path:'',component:ProfileComponent},
    {path:'address/:_id',component:AddressesComponent}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class CustomerRoutingModule { }
