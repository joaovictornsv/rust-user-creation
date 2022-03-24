use sha2::{Sha256, Digest};

pub struct HashProvider {
  secret: String,
}

impl HashProvider {
  pub fn new(secret: String) -> Self {
    HashProvider {
      secret
    }
  }

  pub fn hash(&self, data: &[u8])-> String {
    let mut hasher = Sha256::new();
    
    hasher.update(data);
    hasher.update(self.secret.clone());

    let hash = format!("{:x}", hasher.finalize().clone());

    hash
  }
}

