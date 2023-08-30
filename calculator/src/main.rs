use std::env;


fn main() {
    if let Err(err) = parse_and_calculate() {
        eprintln!("错误: {}", err);
    }
}

fn parse_and_calculate() -> Result<(), String> {
    let (n1, op, n2) = parse_arguments()?;

    match calculate(n1, op, n2) {
        Ok(result) => {
            println!("计算结果为： {}!", result);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn parse_arguments() -> Result<(f64, char, f64), String> {
    let mut args = env::args().skip(1);
    
    let n1 = args.next().ok_or("缺少参数: n1")?.parse().map_err(|_| "无法解析参数: n1为浮点数")?;
    let op = args.next().ok_or("缺少参数: op")?.chars().next().ok_or("无效参数: op")?;
    let n2 = args.next().ok_or("缺少参数: n2")?.parse().map_err(|_| "无法解析参数: n2为浮点数")?;


    Ok((n1, op, n2))
}

fn calculate(num1: f64, op: char, num2: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 != 0.0 {
                Ok(num1 / num2)
            } else {
                Err("除数不能为零".to_string())
            }
        }
        _ => Err("无效参数".to_string())
    }
}
