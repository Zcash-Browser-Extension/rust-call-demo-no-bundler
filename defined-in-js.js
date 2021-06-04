export function name() {
    return 'Rust';
}

export function getInfo(){

    let myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json")

    let raw = JSON.stringify({
      "method": "GetLightdInfo",
      "params": {}
    })

    let requestOptions = {
      method: 'POST',
      headers: myHeaders,
      body: raw,
      redirect: 'follow'
    };

    return fetch("http://23.92.18.222:8000/", requestOptions)
      .then(response => response.json())
      .then(result => result)
      .catch(error => console.log('error', error))  
  }