use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;
use std::collections::HashSet;

fn gen_repeated_numbers(
    max_val: u128,
    min_block: usize,
    max_block: usize,
    min_repeat: usize,
) -> HashSet<u128> {
    let mut out = HashSet::new();

    // número de dígitos máximo que necesitamos manejar
    let max_digits = {
        let mut d = 1usize;
        let mut p = 10u128;
        while p - 1 <= max_val {
            d += 1;
            p *= 10;
            if d > 38 {
                break;
            } // seguridad
        }
        d
    };

    for block_len in min_block..=max_block {
        // límite para k: al menos min_repeat y tal que total_len <= max_digits
        let max_k = max_digits / block_len;
        for k in min_repeat..=max_k {
            // construir números X con block_len dígitos sin ceros líderes
            let start = 10u128.pow((block_len - 1) as u32);
            let end = 10u128.pow(block_len as u32) - 1;

            for x in start..=end {
                // construir la repetición como string y parsear a u128
                let s = x.to_string().repeat(k);
                if let Ok(v) = s.parse::<u128>() {
                    if v <= max_val {
                        out.insert(v);
                    } else {
                        // si v ya excede max_val, para este x y mayores x seguirá excediendo -> romper optimización
                        // pero no es seguro romper el bucle de x aquí porque incrementar x puede aumentar valor,
                        // así que podemos simplemente seguir; romper no es seguro debido al comportamiento decimal.
                    }
                }
            }
        }
    }

    out
}

fn gen_double_numbers(max_val: u128, min_block: usize, max_block: usize) -> HashSet<u128> {
    // especial para k = 2
    let mut out = HashSet::new();

    for block_len in min_block..=max_block {
        let start = 10u128.pow((block_len - 1) as u32);
        let end = 10u128.pow(block_len as u32) - 1;

        for x in start..=end {
            let s = format!("{}{}", x, x);
            if let Ok(v) = s.parse::<u128>() {
                if v <= max_val {
                    out.insert(v);
                }
            }
        }
    }

    out
}

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2025, 2)?;
    let line = input.trim();

    // parse ranges, recoger máximo
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut max_end: u128 = 0;

    for part in line.split(',') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        let (a, b) = p.split_once('-').ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "bad range format")
        })?;
        let start = a
            .parse::<u128>()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "bad number"))?;
        let end = b
            .parse::<u128>()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "bad number"))?;

        if end >= start {
            ranges.push((start, end));
            if end > max_end {
                max_end = end;
            }
        }
    }

    if ranges.is_empty() {
        return Ok(DayResult::new("0".to_string(), "0".to_string()));
    }

    // determinar rangos de bloques: mínimo 1 dígito, máximo la mitad de dígitos de max_end para k>=2,
    // pero para doble (k==2) permitimos block_len up to floor(digits/2)
    let max_digits = max_end.to_string().len();
    let max_block = (max_digits / 2).max(1);

    // Parte 1: k == 2
    let doubles = gen_double_numbers(max_end, 1, max_block);

    // Parte 2: k >= 2
    let repeated = gen_repeated_numbers(max_end, 1, max_block, 2);

    // ahora filtrar por si caen en algún rango (evitamos duplicados usando HashSet)
    let mut seen_p1: HashSet<u128> = HashSet::new();
    let mut seen_p2: HashSet<u128> = HashSet::new();

    for &(s, e) in &ranges {
        for &v in doubles.iter() {
            if v >= s && v <= e {
                seen_p1.insert(v);
            }
        }
        for &v in repeated.iter() {
            if v >= s && v <= e {
                seen_p2.insert(v);
            }
        }
    }

    // sumas
    let sum_p1: u128 = seen_p1.iter().copied().sum();
    let sum_p2: u128 = seen_p2.iter().copied().sum();

    Ok(DayResult::new(sum_p1.to_string(), sum_p2.to_string()))
}
