import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable()
export class PaymentService {

  constructor(private http: HttpClient) { }




  processPaymentResponse(response: any): Observable<any> {
    console.log('Processing payment:', response);
    return this.http.post<any>('http://localhost:3000/payments/capture', response);
  }
}


