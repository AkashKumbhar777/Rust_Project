import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent implements OnInit{
  constructor(private route: ActivatedRoute, private router: Router, private http: HttpClient) { }

  ngOnInit(): void {
    // Use ActivatedRoute's paramMap observable to handle query parameters
    this.route.queryParams.subscribe(params => {
      const code = params['code'];
      if (code) {
        this.handleResponse(code);
      }
    });
  }


  signInOrSignUp() {
    window.location.href = 'https://AKASHPK.b2clogin.com/AKASHPK.onmicrosoft.com/oauth2/v2.0/authorize?p=B2C_1_SignUpSignIn&client_id=1aaf2355-35a8-428c-9bb6-064079ee6c40&nonce=defaultNonce&redirect_uri=http%3A%2F%2Flocalhost%3A4200%2F&scope=openid%20https%3A%2F%2FAKASHPK.onmicrosoft.com%2Fapi%2FTry%26Buy.write%20https%3A%2F%2FAKASHPK.onmicrosoft.com%2Fapi%2FTry%26Buy.read&response_type=code&prompt=login';
  }                        

  handleResponse(code: string) {
    console.log('Handling response with code:', code);
    this.http.post<any>('http://localhost:3000/authenticate', { code })
      .subscribe(
        response => {
          console.log(response);
        },
        error => {
          console.error(error);
        }
      );
  }
}
