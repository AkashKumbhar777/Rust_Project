import { EventEmitter, Injectable } from "@angular/core";
import { Products } from "../models/product.models";
import { HttpClient, HttpErrorResponse, HttpHeaders } from "@angular/common/http";
import { catchError, throwError } from "rxjs";
@Injectable({
    providedIn: 'root'
  })
export class ProductsService{
    url="http://localhost:3000/"
    productSelected=new EventEmitter<Products>();
    singleProduct:Products;
    
    // getHeaders(){
    //     let userStore = localStorage.getItem('admin')
    //     let accessToken = userStore && JSON.parse(userStore).accessToken
    
    //     let httpHeaders: HttpHeaders = new HttpHeaders({
    //       'Content-Type': 'application/json', 
    //       'Authorization': `Bearer ${accessToken}`
    //     })
    
    //     return httpHeaders
    
    //   }
    
      errorHandler(error: HttpErrorResponse){
        console.log(error);  
        return throwError(error)
      }
    constructor(private http: HttpClient){}

   

    //for create new product
    createProduct(newProduct:Products) {
    return this.http.post<Products>(`${this.url}admin/product`, newProduct)
    .pipe(catchError(this.errorHandler))
      }

      getProduct(productId: number){
        return this.http.get<Products>(`${this.url}admin/product/${productId}`)
        .pipe(catchError(this.errorHandler))
      }
    
      getProducts(){
        return this.http.get<Products[]>(`${this.url}admin/products`)
        .pipe(catchError(this.errorHandler))
      }

      updateProduct(productData: Products){
        return this.http.put<Products>(`${this.url}admin/products/${productData.product_id}`, productData)
        .pipe(catchError(this.errorHandler))
      }
      deleteProduct(productId: number){
        return this.http.delete<Products>(`${this.url}admin/productss/${productId}`)
        .pipe(catchError(this.errorHandler))
      }
   
}