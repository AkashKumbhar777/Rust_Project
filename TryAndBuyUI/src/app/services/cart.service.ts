import { EventEmitter, Injectable } from "@angular/core";
import { Products } from "../models/product.models";
import { HttpClient, HttpErrorResponse, HttpHeaders } from "@angular/common/http";
import { catchError, throwError } from "rxjs";
import { Cart } from "../models/dataTypes";

@Injectable({
  providedIn: 'root'
})
export class CartService {
  url = "http://localhost:3000/"

  private cartItems: any[] = [];
  tryCarts: Cart[] = [];
  public cartDataLength = new EventEmitter<Products[] | []>()
  constructor(private http: HttpClient) {

  }
  getHeaders() {
    let userStore = localStorage.getItem('customer')
    let accessToken = userStore && JSON.parse(userStore).accessToken

    let httpHeaders: HttpHeaders = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${accessToken}`
    })
  }

  errorHandler(error: HttpErrorResponse) {
    console.log(error);
    return throwError(error);
  }

  addToLocal(productData: Products) {
    let cartData = []
    let localCart = localStorage.getItem('localCart')
    if (!localCart) {
      localStorage.setItem('localCart', JSON.stringify([productData]))
      this.cartDataLength.emit([productData])
    } else {
      cartData = JSON.parse(localCart)
      cartData.push(productData)
      localStorage.setItem('localCart', JSON.stringify(cartData))
      this.cartDataLength.emit(cartData)
    }
    console.log("inside addtolocal")
    console.log(localCart)
  }
  removeFromLocal(productId: number) {
    let cartData = localStorage.getItem('localCart')
    if (cartData) {
      let items: Products[] = JSON.parse(cartData)
      items = items.filter((item: Products) => productId !== item.product_id)
      localStorage.setItem('localCart', JSON.stringify(items))
      this.cartDataLength.emit(items)
    }

  }

  retrieveFromLocal(): Products[] {
    let localCart = localStorage.getItem('localCart');
    if (localCart) {
      return JSON.parse(localCart) as Products[];
    } else {
      return [];
    }
  }


  addToCart(cart: Cart) {
    const { try_cart_id, ...cartData } = cart; // Destructure cart to remove try_cart_id
    console.log(cartData);
    return this.http.post<Cart>(`${this.url}trycart`, cartData)
      .pipe(catchError(this.errorHandler));
  }


  getAllTryCarts() {
    return this.http.get<Cart>(`${this.url}trycarts`)
      .pipe(catchError(this.errorHandler))
  }

  getTryCartByUserId(userId: number) {
    return this.http.get<Cart>(`${this.url}trycart/${userId}`)
      .pipe(catchError(this.errorHandler))
  }

  deleteTryCart(user_id: number, product_id: number) {
    return this.http.delete(`${this.url}trycart/delete/${user_id}/${product_id}`)
      .pipe(catchError(this.errorHandler))
  }
  updateTryCart(updatedTryCart: Cart) {
    this.http.put<Cart>(`${this.url}trycart/update/${updatedTryCart.user_id}/${updatedTryCart.try_cart_id}`, updatedTryCart)
      .pipe(catchError(this.errorHandler))
  }
}