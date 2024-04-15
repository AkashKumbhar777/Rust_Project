import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ProductsComponent } from './products/products.component';
import { ProductAddComponent } from './product-add/product-add.component';
import { ProductUpdateComponent } from './product-update/product-update.component';

const routes: Routes = [
  {path: 'products', children:[
    {path: '', component: ProductsComponent},
    {path: 'add', component: ProductAddComponent},
    {path: 'update/:_id', component: ProductUpdateComponent}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class SellerRoutingModule { }
