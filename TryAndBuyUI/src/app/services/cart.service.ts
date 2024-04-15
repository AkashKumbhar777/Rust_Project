import { EventEmitter, Injectable } from "@angular/core";
import { Products } from "../models/product.models";
import { HttpClient, HttpErrorResponse, HttpHeaders } from "@angular/common/http";
import { throwError } from "rxjs";
import { Cart } from "../models/dataTypes";

@Injectable({
    providedIn: 'root'
  })
export class CartService{
    url="http://localhost:3000/"

    private cartItems: any[] = [];
    tryCarts: Cart[] = [];
    public cartDataLength = new EventEmitter<Products[] | []>()
    constructor(private http: HttpClient){

    }
    getHeaders(){
        let userStore = localStorage.getItem('customer')
        let accessToken = userStore && JSON.parse(userStore).accessToken
    
        let httpHeaders: HttpHeaders = new HttpHeaders({
          'Content-Type': 'application/json', 
          'Authorization': `Bearer ${accessToken}`
        })
    }

    errorHandler(error: HttpErrorResponse){
        console.log(error);  
        return throwError(error);
      }

      addToLocal(productData: Products){
        let cartData = []
        let localCart = localStorage.getItem('localCart')
        if(!localCart){
          localStorage.setItem('localCart', JSON.stringify([productData]))
          this.cartDataLength.emit([productData])
        }else{
          cartData = JSON.parse(localCart)
          cartData.push(productData)
          localStorage.setItem('localCart', JSON.stringify(cartData))
          this.cartDataLength.emit(cartData)
        }
        console.log("inside addtolocal")
        console.log(localCart)
      }
      removeFromLocal(productId: number){
        let cartData = localStorage.getItem('localCart')
        if(cartData){
          let items:Products[] = JSON.parse(cartData)
          items = items.filter((item:Products)=>productId!==item.product_id)
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
    
    addToCart(product:Products){
        // const existingPrdt=this.cartItems.find(item=>item.id===product._id);
        // if(!existingPrdt){
        //     this.cartItems.push(product);
        // }

    }

    getAllTryCarts() {
      this.http.get<Cart[]>('http://your-api-url.com/get_all_try_carts')
        .subscribe(
          (tryCarts: Cart[]) => {
            this.tryCarts = tryCarts;
          },
          (error) => {
            console.error('Error fetching try carts:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }

    getTryCartByUserId(userId:string) {
      this.http.get<Cart[]>(`http://your-api-url.com/get_try_cart_by_user_id/${userId}`)
        .subscribe(
          (tryCarts: Cart[]) => {
            this.tryCarts = tryCarts;
          },
          (error) => {
            console.error('Error fetching try carts:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }
    deleteTryCart(tryCartIdToDelete:string) {
      this.http.delete(`http://your-api-url.com/delete_try_cart/${tryCartIdToDelete}`)
        .subscribe(
          () => {
            console.log('Try cart deleted successfully');
            // Optionally, you can perform some action after successful deletion
          },
          (error) => {
            console.error('Error deleting try cart:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }
    updateTryCart(tryCartIdToUpdate:string,updatedTryCart:Cart) {
      this.http.put<Cart>(`http://your-api-url.com/update_try_cart/${tryCartIdToUpdate}`, updatedTryCart)
        .subscribe(
          (updatedTryCart: Cart) => {
            console.log('Try cart updated successfully:', updatedTryCart);
            // Optionally, you can perform some action after successful update
          },
          (error) => {
            console.error('Error updating try cart:', error);
            // Handle error appropriately, e.g., show error message to the user
          }
        );
    }
}