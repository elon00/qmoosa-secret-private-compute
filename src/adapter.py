"""Secret Network Confidential Smart Contract Engine."""
import hashlib

def create_viewing_key(account: str, entropy: str) -> str:
    return hashlib.sha256(f"viewing_key:{account}:{entropy}".encode()).hexdigest()

def encrypt_contract_payload(payload: dict, viewing_key: str) -> dict:
    import json
    raw = json.dumps(payload, sort_keys=True).encode()
    enc = hashlib.sha256(viewing_key.encode() + raw).hexdigest()
    return {"ciphertext": enc, "nonce": hashlib.sha256(raw).hexdigest()[:16]}
