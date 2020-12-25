fn mul_mod_magic(value: u64, subject: u64) -> u64 {
    const MAGIC_NUMBER: u64 = 20_201_227;
    (value * subject) % MAGIC_NUMBER
}

fn find_loop_size(subject: u64, pub_key: u64) -> Option<u64> {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        value = mul_mod_magic(value, subject);
        loop_size += 1;

        if value == pub_key {
            return Some(loop_size);
        }
    }
}

fn perform_handshake(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = mul_mod_magic(value, subject);
    }
    value
}

fn find_encryption_key(card_pub: u64, door_pub: u64) -> Option<u64> {
    const CARD_SUBJECT: u64 = 7;
    const DOOR_SUBJECT: u64 = 7;

    let card_loop_size = find_loop_size(CARD_SUBJECT, card_pub)?;
    let encryption_key = perform_handshake(door_pub, card_loop_size);

    #[cfg(debug_assertions)]
    {
        let door_loop_size = find_loop_size(DOOR_SUBJECT, door_pub)?;
        assert_eq!(perform_handshake(card_pub, door_loop_size), encryption_key);
    }

    Some(encryption_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            find_encryption_key(5_764_801, 17_807_724).expect("failed to solve"),
            14_897_079
        );
    }
}
