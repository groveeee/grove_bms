
#[cfg(test)]
mod tests{
    use std::thread::sleep;
    use chrono::{Duration, Local, Utc};
    use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
    use crate::utils::jwt::Claims;

    #[test]
    fn test_jwt(){
        println!("test jwt");
        let now = Local::now();
        let expire = (now + Duration::seconds(3)).timestamp();
        println!("{}", expire);
        let claims = Claims {
            aud: "me".to_owned(),
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: expire as u64,
        };
        let  token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_bytes())).unwrap();
        sleep(Duration::seconds(4).to_std().unwrap());
        let time = Utc::now();
        println!("{}", time.timestamp());
        println!("{}", token);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["me"]);
        validation.set_required_spec_claims(&["exp", "sub", "aud"]);
        // 在令牌之前的一段时间（以秒为单位）拒绝令牌， exp 以防止在通过网络传输时过期
        validation.reject_tokens_expiring_in_less_than = 60;
        let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation)
         {
            Ok(c) => {c}
             Err(err) => {
                 let kind = err.into_kind();
                 println!("{:?}", kind);
                 panic!("err");
             }
        };
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);
    }
}