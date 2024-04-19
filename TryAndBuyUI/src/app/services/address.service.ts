import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { catchError, throwError } from "rxjs";
import { address } from "../models/dataTypes";

@Injectable({
  providedIn: 'root'
})
export class AddressService {
  url = "http://localhost:3000/"

  constructor(private http: HttpClient) { }

  errorHandler(error: HttpErrorResponse) {
    console.log(error);
    return throwError(error)
  }

  //creating address
  createAddress(newAddress: address) {
    const { address_id, ...addnewAddressress } = newAddress;
    return this.http.post<address>(`${this.url}address`, addnewAddressress)
      .pipe(catchError(this.errorHandler))
  }

  //get address by id
  getAddress(userId: number) {
    return this.http.get<address>(`${this.url}address/${userId}`)
      .pipe(catchError(this.errorHandler))
  }

  //get all address
  getAddresses() {
    return this.http.get<address[]>(`${this.url}addresses`)
      .pipe(catchError(this.errorHandler))
  }

  //update address
  updateAddress(adrsData: address) {
    return this.http.put<address>(`${this.url}address/update/${adrsData.address_id}`, adrsData)
      .pipe(catchError(this.errorHandler))
  }

  //delete product
  deleteAddress(adrsId: number) {
    return this.http.delete<address>(`${this.url}address/delete/${adrsId}`)
      .pipe(catchError(this.errorHandler))
  }
}