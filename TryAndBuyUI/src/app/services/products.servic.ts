import { EventEmitter, Injectable } from "@angular/core";
import { Products } from "../models/product.models";
import { HttpClient, HttpErrorResponse, HttpHeaders } from "@angular/common/http";
import { catchError, throwError } from "rxjs";
@Injectable({
  providedIn: 'root'
})
export class ProductsService {
  url = "http://localhost:3000/"
  productSelected = new EventEmitter<Products>();
  singleProduct: Products;

  errorHandler(error: HttpErrorResponse) {
    console.log(error);
    return throwError(error)
  }
  constructor(private http: HttpClient) { }



  //for create new product
  createProduct(newProduct: Products) {
    return this.http.post<Products>(`${this.url}admin/product`, newProduct)
      .pipe(catchError(this.errorHandler))
  }

  //get product by id
  getProduct(productId: number) {
    return this.http.get<Products>(`${this.url}admin/product/${productId}`)
      .pipe(catchError(this.errorHandler))
  }

  //get all products
  getProducts() {
    return this.http.get<Products[]>(`${this.url}admin/products`)
      .pipe(catchError(this.errorHandler))
  }

  //update product
  updateProduct(productData: Products) {
    return this.http.put<Products>(`${this.url}admin/products/${productData.product_id}`, productData)
      .pipe(catchError(this.errorHandler))
  }

  //delete product
  deleteProduct(productId: number) {
    return this.http.delete<Products>(`${this.url}admin/productss/${productId}`)
      .pipe(catchError(this.errorHandler))
  }

}