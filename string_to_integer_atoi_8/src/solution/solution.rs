pub struct Solution;

enum States {
    Init,
    Decoding
}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut current_state = States::Init;
        let mut number_chars_vec: Vec<char> = vec![];
        for c in s.chars() {
            match current_state {
                States::Init => {
                    if (c >= '0' && c <= '9') || c == '-' || c == '+' {
                        number_chars_vec.push(c);
                        current_state = States::Decoding;
                    } else if c == ' ' {
                        continue;
                    } else {
                        break;
                    }
                },
                States::Decoding => {
                    if c >= '0' && c <= '9' {
                        number_chars_vec.push(c);
                    } else {
                        break;
                    }
                }
            }
        }
        // Criar iterador peekable e verificar sinal com peek
        let mut chars_iter = number_chars_vec.iter().peekable();
        let is_negative = {
            let primeiro = chars_iter.peek();
            let is_negative = primeiro.map(|&&c| c == '-').unwrap_or(false);
            // Consumir o sinal se presente
            if primeiro.map(|&&c| c == '-' || c == '+').unwrap_or(false) {
                chars_iter.next();
            }
            is_negative
        };
        
        // Construir o número como positivo usando i32 com verificação de overflow
        let mut result: i32 = 0;
        for &c in chars_iter {
            let digit = (c as i32) - ('0' as i32);
            
            // Multiplicar por 10 e adicionar dígito, verificando overflow
            result = match result.checked_mul(10).and_then(|val| val.checked_add(digit)) {
                Some(val) => val,
                None => return if is_negative { i32::MIN } else { i32::MAX },
            };
        }
        
        // Aplicar o sinal
        if is_negative {
            match result.checked_mul(-1) {
                Some(val) => val,
                None => i32::MIN, // -i32::MAX - 1 causaria overflow, retorna MIN
            }
        } else {
            result
        }
    }
}