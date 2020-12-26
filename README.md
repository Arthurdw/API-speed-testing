# NodeJS ([Express](http://expressjs.com/)) vs Rust ([Rocket](http://rocket.rs)) API

## Stress testing

We tested both of the API's using Artillery. During the stress testing we let it send 20.000 requests per second, and each request had to create a random token that has a length of 64 characters.  
If you want to see all the specific outputs you can find it in the `Artillery (Load Tester)` folder.

### Comparing Stress Test Results

#### Total elapsed time

```js
Rust (Rocket): 7 minutes, 9 seconds
JavaScript: 7 minutes, 42 seconds
Python: 8 minutes, 22 seconds
Go: 8 minutes, 45 seconds
Rust (tower-web): 6 minutes, 13 seconds
Rust (actix-web): 5 minutes, 50 seconds
```

#### Requests

##### Total completed requests

```js
JavaScript: 200000
Rust (Rocket): 200000
Python: 392
Go: 16312
Rust (tower-web): 16269
Rust (actix-web): 200000
```

##### Average requests handled per second

```js
JavaScript: 431.82
Rust (Rocket): 465.41
Python: 0.7
Go: 380.34
Rust (tower-web): 536.24
Rust (actix-web): 568.88
```

##### Response times

###### Minimum response time (msec)

```js
JavaScript: 1.2
Rust (Rocket): 1
Python: 9.6
Go: 5
Rust (tower-web): 4
Rust (actix-web): 0.9
```

###### Maximum response time (msec)

```js
JavaScript: 183.6
Rust (Rocket): 312.5
Python: 2205
Go: 2331.3
Rust (tower-web): 133.7
Rust (actix-web): 112.9
```

###### Median response time (msec)

```js
JavaScript: 9
Rust (Rocket): 8.2
Python: 664.6
Go: 13.6
Rust (tower-web): 8.4
Rust (actix-web): 6.5
```

## Letting it generate big tokens

As the stress testing was used for the most likely scenario (by generating random tokens that are have a length of 64 character) we also want to test heavy server-side token generation, and check what will be the fastest.  
All our data is displayed in milliseconds.

### Comparing token generation Results

#### 64 characters

```js
JavaScript: 1.32
Rust (Rocket): 203
Go: 0.64
Rust (tower-web): 0.48
Rust (actix-web): 0.48
```

#### 128 characters

```js
JavaScript: 1.87
Rust (Rocket): 203
Go: 0.7
Rust (tower-web): 0.48
Rust (actix-web): 0.69
```

#### 256 characters

```js
JavaScript: 1.17
Rust (Rocket): 204
Go: 0.75
Rust (tower-web): 0.45
Rust (actix-web): 0.64
```

#### 512 characters

```js
JavaScript: 1.24
Rust (Rocket): 204
Go: 0.77
Rust (tower-web): 0.52
Rust (actix-web): 0.5
```

#### 1 024 characters

```js
JavaScript: 1.20
Rust (Rocket): 206
Go: 2.63
Rust (tower-web): 0.54
Rust (actix-web): 0.66
```

#### 2 048 characters

```js
JavaScript: 1.48
Rust (Rocket): 201
Go: 4.01
Rust (tower-web): 0.51
Rust (actix-web): 0.57
```

#### 4 096 characters

```js
JavaScript: 2.37
Rust (Rocket): 204
Go: 5.77
Rust (tower-web): 0.65
Rust (actix-web): 0.67
```

#### 8 192 characters

```js
JavaScript: 2.29
Rust (Rocket): 203
Go: 18.7
Rust (tower-web): 0.67
Rust (actix-web): 0.6
```

#### 16 384 characters

```js
JavaScript: 4.43
Rust (Rocket): 214
Go: 57
Rust (tower-web): 0.73
Rust (actix-web): 0.82
```

### Even bigger tokens

#### 100 000 characters

```js
JavaScript: 20
Rust (Rocket): 221
Go: 4269.65
Rust (tower-web): 4.7
Rust (actix-web): 2.33
```

#### 500 000 characters

```js
JavaScript: 92
Rust (Rocket): 217
Go: 116775.545
Rust (tower-web): 8.91
Rust (actix-web): 7.26
```

#### 1 000 000 characters

```js
JavaScript: 219
Rust (Rocket): 246
Go: 226980.849
Rust (tower-web): 19.7
Rust (actix-web): 15
```

#### 100 000 000 characters

```js
JavaScript: CRASH
Rust (Rocket): 3621
Go: NO RESPONSE
Rust (tower-web): 1480
Rust (actix-web): 1646
```

#### 1 000 000 000 characters

```js
JavaScript: CRASH
Rust (Rocket): 34985
Go: NO RESPONSE
Rust (tower-web): 17355
Rust (actix-web): 15225
```

#### 1 000 000 000 000 characters

```js
JavaScript: CRASH
Rust (Rocket): CRASH
Go: NO RESPONSE
Rust (tower-web): CRASH (mem allocation failed)
Rust (actix-web): CRASH (mem allocation failed)
```

## Results

The results from the character generation are definitely shocking, I expected Rust to be faster in all of them, yet this was not the case. Rust (Rocket) did handle server side progressing better yet its standard response time is around 200ms.  
The latest tests (character generation) were performed using Insomnia.  

Rust (Rocket) does perform better for enterprise applications as we can see by the stress test.

Rust (tower-web) is amazing for its speed and can definitly be used for big API's, but Rust (actix-web) is more optimalized with its easy to use asynchronous implementation

GO is an excellent solution for fast non intensive server processing. Yet Rust would be the better option for more heavy server sided processing. 

& don't make heavy used API's in Python :3



I would currently definitly reccomend using Actix Web with rust as it is super fast and very reliable. If you don't know rust I would reccomend Go for small API's and Javascript for the larger ones. (Or you could learn Rust and benefit from its performance and memory safety)