use crate::response::get_base_url;
use crate::types::VerifiedManifest;
use rocket::http::Header;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

impl<'r> Responder<'r> for VerifiedManifest {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        //The front end is responsible for assembling URLs, backend should deal in arguments
        let location = format!(
            "{}/v2/{}/manifests/{}",
            get_base_url(req),
            self.repo_name(),
            self.tag()
        );
        let location_header = Header::new("Location", location);
        let digest = Header::new("Docker-Content-Digest", self.digest().to_string());
        Response::build()
            .status(Status::Created)
            .header(location_header)
            .header(digest)
            .ok()
    }
}

#[cfg(test)]
mod test {
    use crate::response::test_helper::test_route;
    use crate::types::{create_verified_manifest, Digest, RepoName};
    use rocket::http::Status;

    #[test]
    fn accepted_ok() {
        let response = test_route(create_verified_manifest(
            RepoName("repo_name".to_string()),
            Digest("digest".to_string()),
            "ref".to_string(),
            "can't_remember".to_string(),
        ));
        assert_eq!(response.status(), Status::Created);
    }
}
