export class Products{
    product_id: number;
    product_name: string;
        price: number;
        specifications: string;
        product_description: string;
        image_url: string;
        updated_at: string;
        created_at: string;

        constructor(product_id : number,
            product_name: string,
            price: number,
            specifications: string,
            product_description: string,
            image_url: string,
            updated_at: string,
            created_at: string ){
                this.product_id=product_id;
                this.product_name=product_name;
                this.price=price;
                this.specifications=specifications;
                this.product_description=product_description;
                this.image_url=image_url;
                this.updated_at=updated_at;
                this.created_at=created_at;
            }
    
}