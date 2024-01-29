use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use request::response::Response;
use server::ThreadPool;
use std::collections::HashMap;



fn main() {
    let listener =
       TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool :: new(4);

    let mut map = 
    HashMap::new ();
   
      map.insert("chave1",1);
      map.insert("chaver2",2);

      println!("{:?}",map);

      blocking_get().unwrap();
  
   // let user_credentials: HashMap<String, String> = var_name;
   let mut users: HashMap<String, String> = HashMap::new();

    // Adiciona alguns usuários para fins de exemplo
    users.insert(String::from("usuario1"), String::from("senha1"));
    users.insert(String::from("usuario2"), String::from("senha2"));


  for stream in listener.incoming() {
      let stream = stream.unwrap();
      pool.execute(|| {
          handle_connection(stream);
      });


    for stream in listener.incoming().take(2){
      let stream = stream.unwrap();
  
      pool.execute ( || {
        handle_connection(stream);
      });    
    }
    println!("Shutting down.");
}

fn handle_connection( mut stream: TcpStream) {
     let mut buffer = [0;1024];

     stream.read(&mut buffer).unwrap();

    let get = b"GET /login HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post = b"POST/add_user HTTP/1.1\r\n";

    let (status_line, filename) = 
      if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")

    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
       

    };

    
    let contents =
      fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
fn handle_login(stream: &mut TcpStream, users: &HashMap<String, String>, request: &str) {
  // Extrai o nome de usuário e senha da solicitação
  let parts: Vec<&str> = request.split("\r\n").collect();
  let credentials: Vec<&str> = parts[0].split_whitespace().collect();
  let username = credentials[2];
  let password = credentials[4];

  // Verifica se as credenciais estão corretas
  if let Some(expected_password) = users.get(username) {
      if *expected_password == password {
          let response = "HTTP/1.1 200 OK\r\n\r\nLogin Successful!";
          stream.write(response.as_bytes()).unwrap();
          stream.flush().unwrap();
          return;
      }
  }

  // Se as credenciais não forem encontradas ou não corresponderem, retorna um erro de login
  let response = "HTTP/1.1 401 UNAUTHORIZED\r\n\r\nLogin Failed!";
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

// Função para lidar com solicitações de adição de novos usuários
fn handle_add_user(stream: &mut TcpStream, users: &mut HashMap<String, String>, request: &str) {
  // Extrai o nome de usuário e senha da solicitação
  let parts: Vec<&str> = request.split("\r\n").collect();
  let credentials: Vec<&str> = parts[0].split_whitespace().collect();
  let username = credentials[2];
  let password = credentials[4];
    // Verificar se o método HTTP é POST e o conteúdo contém os parâmetros de usuário
    if lines.len() > 0 && lines[0].starts_with("POST /add_user HTTP/1.1") {
      let params: Vec<&str> = lines[0].split_whitespace().collect();
      if params.len() > 1 {
          let user_data: Vec<&str> = params[1].split('&').collect();
          for data in user_data {
              let pair: Vec<&str> = data.split('=').collect();
              if pair.len() == 2 {
                  let key = pair[0];
                  let value = pair[1];
                  println!("{}: {}", key, value) ;
              }
          }
      }
  }

  // Adiciona o novo usuário ao HashMap
  users.insert(username.to_string(), password.to_string());

  let response = "HTTP/1.1 200 OK\r\n\r\nUser Added Successfully!";
  stream.write(response.as_bytes()).unwrap() ;
  stream.flush().unwrap();
}
}


