import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { catchError, throwError } from "rxjs";
import { BuyCart, Order } from "../models/dataTypes";

@Injectable({
  providedIn: 'root'
})
export class BuyCartService {
  url = "http://localhost:3000/"

  constructor(private http: HttpClient) { }

  errorHandler(error: HttpErrorResponse) {
    console.log(error);
    return throwError(error)
  }

  //create buy cart
  addToBuyCart(cart: BuyCart) {
    const { buy_cart_id, ...cartData } = cart; // Destructure cart to remove try_cart_id
    console.log(cartData);
    return this.http.post<BuyCart>(`${this.url}buycart`, cartData)
      .pipe(catchError(this.errorHandler));
  }

  //get all buy cart
  getAllBuyCarts() {
    return this.http.get<BuyCart>(`${this.url}buycarts`)
      .pipe(catchError(this.errorHandler))
  }

  //get order by UserId
  getBuyCartbyId(id: number) {
    return this.http.get<BuyCart>(`${this.url}buycart/${id}`)
      .pipe(catchError(this.errorHandler))
  }

  //update order by userid
  updateOrder(data: BuyCart) {
    return this.http.put<BuyCart>(`${this.url}buycart/update/${data.buy_cart_id}/${data.user_id}`, { data })
      .pipe(catchError(this.errorHandler))
  }

  //delete order
  deleteOrder(id: number) {
    return this.http.delete<BuyCart>(`${this.url}buycart/delete/${id}`)
      .pipe(catchError(this.errorHandler))
  }

}