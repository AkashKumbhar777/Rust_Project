import { EventEmitter, Injectable } from "@angular/core";
import { ProductsService } from "./products.servic";
import { Products } from "../models/product.models";
import { HttpClient, HttpErrorResponse, HttpHeaders } from "@angular/common/http";
import { BehaviorSubject, catchError, throwError } from "rxjs";
import { Cart, checkOut } from "../models/dataTypes";

@Injectable({
    providedIn: 'root'
  })
export class ShopService{
    public url=''
    product:Products
    public cartDataLength = new EventEmitter<Products[] | []>()
    private selectedProductsSubject = new BehaviorSubject<Products[]>([]);
    selectedProducts$ = this.selectedProductsSubject.asObservable();
    constructor(private productSer:ProductsService,private http: HttpClient){}
    getHeaders(){
      let userStore = localStorage.getItem('customer')
      let accessToken = userStore && JSON.parse(userStore).accessToken
  
      let httpHeaders: HttpHeaders = new HttpHeaders({
        'Content-Type': 'application/json', 
        'Authorization': `Bearer ${accessToken}`
      })
  
      return httpHeaders
  
    }

    errorHandler(error: HttpErrorResponse){
        console.log(error);  
        return throwError(error)
      }
    getproductdetails(productId : string){
        // return this.http.get<Products>(`${this.url}products/${productId}`)
        // .pipe(catchError(this.errorHandler))

    }
    getCart(){
      let Headers = this.getHeaders()
      return this.http.get<Cart>(`${this.url}carts/get-cart`, { headers: Headers })
      .pipe(catchError(this.errorHandler))
    }
    getCartCount(){
      let Headers = this.getHeaders()
      return this.http.get<any>(`${this.url}carts/get-cart`, { headers: Headers })
      .pipe(catchError(this.errorHandler))
      .subscribe((res)=>{
        if(res){
          this.cartDataLength.emit(res.cart.products)
        }
      })
    }

    setSelectedProducts(products: Products[]) {
      this.selectedProductsSubject.next(products);
    }

    
}