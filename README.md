# NodeJS ([Express](http://expressjs.com/)) vs Rust ([Rocket](http://rocket.rs)) API

## Stress testing

We tested both of the API's using Artillery. During the stress testing we let it send 20.000 requests per second, and each request had to create a random token that has a length of 64 characters.  
If you want to see all the specific outputs you can find it in the `Artillery (Load Tester)` folder.

### Comparing Stress Test Results

#### Total elapsed time

```js
JavaScript: 7 minutes, 42 seconds
Rust: 7 minutes, 9 seconds
Python: 8 minutes, 22 seconds
Go: 8 minutes, 45 seconds
```

#### Requests

##### Total completed requests

```js
JavaScript: 200000
Rust: 200000
Python: 392
Go: 16312
```

##### Average requests handled per second

```js
JavaScript: 431.82
Rust: 465.41
Python: 0.7
Go: 380.34
```

##### Response times

###### Minimum response time (msec)

```js
JavaScript: 1.2
Rust: 1
Python: 9.6
Go: 5
```

###### Maximum response time (msec)

```js
JavaScript: 183.6
Rust: 312.5
Python: 2205
Go: 2331.3
```

###### Median response time (msec)

```js
JavaScript: 9
Rust: 8.2
Python: 664.6
Go: 13.6
```

## Letting it generate big tokens

As the stress testing was used for the most likely scenario (by generating random tokens that are have a length of 64 character) we also want to test heavy server-side token generation, and check what will be the fastest.  
All our data is displayed in milliseconds.

### Comparing token generation Results

#### 64 characters

```js
JavaScript: 1.32
Rust: 203
Go: 0.64
```

#### 128 characters

```js
JavaScript: 1.87
Rust: 203
Go: 0.7
```

#### 256 characters

```js
JavaScript: 1.17
Rust: 204
Go: 0.75
```

#### 512 characters

```js
JavaScript: 1.24
Rust: 204
Go: 0.77
```

#### 1 024 characters

```js
JavaScript: 1.20
Rust: 206
Go: 2.63
```

#### 2 048 characters

```js
JavaScript: 1.48
Rust: 201
Go: 4.01
```

#### 4 096 characters

```js
JavaScript: 2.37
Rust: 204
Go: 5.77
```

#### 8 192 characters

```js
JavaScript: 2.29
Rust: 203
Go: 18.7
```

#### 16 384 characters

```js
JavaScript: 4.43
Rust: 214
Go: 57
```

### Even bigger tokens

#### 100 000 characters

```js
JavaScript: 20
Rust: 221
Go: 4269.65
```

#### 500 000 characters

```js
JavaScript: 92
Rust: 217
Go: 116775.545
```

#### 1 000 000 characters

```js
JavaScript: 219
Rust: 246
Go: 226980.849
```

#### 100 000 000 characters

```js
JavaScript: CRASH
Rust: 3621
Go: NO RESPONSE
```

#### 1 000 000 000 characters

```js
JavaScript: CRASH
Rust: 34985
Go: NO RESPONSE
```

#### 1 000 000 000 000 characters

```js
JavaScript: CRASH
Rust: CRASH
Go: NO RESPONSE
```

## Results

The results from the character generation are definitely shocking, I expected Rust to be faster in all of them, yet this was not the case. Rust did handle server side progressing better yet its standard response time is around 200ms.  
The latest tests (character generation) were performed using Insomnia.  

Rust does perform better for enterprise applications as we can see by the stress test.

GO is an excellent solution for fast non intensive server processing. Yet Rust would be the better option for more heavy server sided processing. 

& don't make heavy used API's in Python :3