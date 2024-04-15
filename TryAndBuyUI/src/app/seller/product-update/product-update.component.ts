import { Component, OnInit } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { ProductsService } from '../../services/products.servic';
import { Products } from '../../models/product.models';

@Component({
  selector: 'app-product-update',
  templateUrl: './product-update.component.html',
  styleUrl: './product-update.component.css'
})
export class ProductUpdateComponent implements OnInit{

  isDisabled: boolean = true; 
  public productMsg: string | undefined
  public productId: number | undefined
  public product: Products | undefined

  constructor(private fb: FormBuilder, private productService: ProductsService, private router: Router, 
    private activatedRoute: ActivatedRoute){}

  productUpdateForm = this.fb.group({
    product_name: [''],
    product_description:[''],
    price: [0],
    image_url: [''],
    specifications: [''],
    updated_at: [''],
    created_at:['']
  })
  ngOnInit(): void {
    this.activatedRoute.params.subscribe((params)=>{
      this.productId = parseInt(params?.['_id'],10);
      console.log(params?.['_id']);
      this.productId && this.productService.getProduct(this.productId).subscribe((res)=>{
        console.log(res);
        console.log(this.productId);
      
        if(res){
          this.productUpdateForm = this.fb.group({
            product_name: [res.product_name],
            product_description: [res.product_description],
            price: [res.price],
            image_url: [res.image_url],
            specifications: [res.specifications],
            updated_at:[res.updated_at],
            created_at:[res.created_at]
          })
        }
      })
    })
  }
  updateProduct(){
    
    let productData = this.productUpdateForm.value as Products
    if(this.productId){
      productData.product_id = this.productId
    }  
    this.productService.updateProduct(productData).subscribe((res)=>{
      if(res){
        console.log(productData);
        this.productMsg = 'Product Has Been Successfully Updated'
      } 
      this.getTimeout('suc')
    }, (err)=>{
      if(err){
        this.productMsg = err.statusText
      }
      this.getTimeout('err')
    })
  }

  getTimeout(val: string){
    if(val==='suc'){
      setTimeout(() => {
        this.productMsg = undefined
        this.router.navigate(['/products'])
      }, 2500);
    }else{
      setTimeout(() => {
        this.productMsg = undefined
      }, 4000);
    }
  }
  
}
