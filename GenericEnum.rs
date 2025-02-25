fn main(){
    let mut _res = Result1::Success::<u32,u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u32, 'd'); 
    //It will get an error, in the declaration "FailureCode"
    //is expected to be u16, and after it is assigned to
    //be u32
}
enum Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode, char),
    Uncertainty,
}
