export interface Login{
    login_id:number,
    username:string, 
    password:string, 
    user_role: 'user' | 'admin',
    last_logged_in:Date
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
    productId: string, 
    _id: string, 
    title: string, 
    price: number, 
    color: string, 
    categories: string, 
    desc: string, 
    image: string, 
    size: string, 
    quantity?: number | undefined, 
    cart?: any,
    cartCount?: number | undefined
}

export interface PriceSummary{
    price: number,
    discount: number,
    tax: number,
    delivery: number,
    total: number
}

export interface Order{
    email: string,
    address: string,
    contact: string,
    totalPrice: number,
    cartTotal: number,
    paymentIntent: {
        id: string,
        amount: number
    },
    orderStatus: string

}

export interface checkOut{
    _id: string,
    quantity: number
}