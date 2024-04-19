export interface Login{
     user_id: number,
     first_name: String,
     last_name: String,
     email: String,
     phone: String,
     profile_picture: string,
     created_at: String,
     updated_at: String,
     user_role:String
}
// export interface Product{
//     _id: string,
//     title: string,
//     price: number,
//     color: string,
//     categories: string,
//     desc: string,
//     image: string,
//     size: string,
//     quantity?: number,
//     productId: string
// }


export interface Cart{ 
    try_cart_id: number,
    user_id: number,
    product_id: number,
    added_at: String,
}

export interface PriceSummary{
    price: number,
    discount: number,
    tax: number,
    delivery: number,
    total: number
}

export interface Order{
    order_id:number,
    user_id:number,
    address_id: number,
    product_id: number,
    total_amount: number,
    order_status:String,
    order_date:String

}

export interface BuyCart {
    buy_cart_id: number,
    user_id: number,
    product_id: number,
    quantity: number,
    total_amount: number
}

export interface checkOut{
    _id: string,
    quantity: number
}

export interface address{
    address_id: number,
    user_id: number,
    address_line1: String,
    address_line2: String,
    city: String,
    add_state: String,
    postal_code: String,
    country: String,
}