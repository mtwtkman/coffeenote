pub mod production_area;
pub mod region;

pub trait Usecase {
    type Request;
    type Response;
    type Error;

    fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
