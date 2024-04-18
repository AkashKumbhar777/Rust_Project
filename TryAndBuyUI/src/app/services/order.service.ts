import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { catchError, throwError } from "rxjs";
import { Order } from "../models/dataTypes";

@Injectable({
  providedIn: 'root'
})
export class OrderService {
  url = "http://localhost:3000/"

  constructor(private http: HttpClient) { }
  errorHandler(error: HttpErrorResponse) {
    console.log(error);
    return throwError(error)
  }
  addToOrder(order: Order) {
    const { order_id, ...cartData } = order;
    console.log(cartData);
    return this.http.post<Order>(`${this.url}order`, cartData)
      .pipe(catchError(this.errorHandler));
  }


  //get all orders
  getAllOrders() {
    return this.http.get<Order>(`${this.url}orders`)
      .pipe(catchError(this.errorHandler))
  }

  //get order by UserId
  getOrderbyId(id: number) {
    return this.http.get<Order>(`${this.url}orders/${id}`)
      .pipe(catchError(this.errorHandler))
  }

  //update order by userid
  updateOrder(data: Order) {
    return this.http.put<Order>(`${this.url}orders/update/${data.order_id}`, { data })
      .pipe(catchError(this.errorHandler))
  }

  //delete order
  deleteOrder(id: number) {
    return this.http.delete<Order>(`${this.url}order/delete/${id}`)
      .pipe(catchError(this.errorHandler))
  }
}