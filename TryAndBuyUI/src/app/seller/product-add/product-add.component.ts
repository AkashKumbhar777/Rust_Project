import { Component } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { Router } from '@angular/router';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';

@Component({
  selector: 'app-product-add',
  templateUrl: './product-add.component.html',
  styleUrl: './product-add.component.css'
})
export class ProductAddComponent {
  public productMsg: string | undefined
  constructor(private fb: FormBuilder, private productService: ProductsService, private router: Router){}
  productForm = this.fb.group({
    // product_id:[''],
    product_name: [''],
    product_description:[''],
    price: [0],
    image_url: [''],
    specifications: [''],
    created_at: [''],
    updated_at: ['']
  })
  onSubmit(){
    console.log(this.productForm);
    let productData = this.productForm!.value as Products
    this.productService.createProduct(productData).subscribe(
      (res: any) => {
        console.log(res);
        if (res) {
          this.productMsg = 'Product is successfully added';
          this.router.navigate(['/products']);
        }
        this.getTimeout();
      },
      (err: any) => {
        if (err) {
          this.productMsg = 'Please Add Unique Name Or Add A Valid Price';
        }
        this.getTimeout();
      }
    );
    }    
  getTimeout(){
    setTimeout(() => {
      this.productMsg = undefined
      this.productForm.reset()
    }, 4000);
  }
}
