# NodeJS ([Express](http://expressjs.com/)) vs Rust ([Rocket](http://rocket.rs)) API

## Stress testing

We tested both of the API's using Artillery. During the stress testing we let it send 20.000 requests per second, and each request had to create a random token that has a length of 64 characters.  
If you want to see all the specific outputs you can find it in the `Artillery (Load Tester)` folder.

### Comparing Stress Test Results

#### Total elapsed time

```js
JavaScript: 7 minutes, 42 seconds
Rust: 7 minutes, 9 seconds
```

#### Requests

##### Total completed requests

```js
JavaScript: 200000
Rust: 200000
```

##### Average requests handled per second

```js
JavaScript: 431.82
Rust: 465.41
```

##### Response times

###### Minimum response time (msec)

```js
JavaScript: 1.2
Rust: 1
```

###### Maximum response time (msec)

```js
JavaScript: 183.6
Rust: 312.5
```

###### Median response time (msec)

```js
JavaScript: 9
Rust: 8.2
```

## Letting it generate big tokens

As the stress testing was used for the most likely scenario (by generating random tokens that are have a length of 64 character) we also want to test heavy server-side token generation, and check what will be the fastest.  
All our data is displayed in milliseconds.

### Comparing token generation Results

#### 64 characters

```js
JavaScript: 1.32
Rust: 203
```

#### 128 characters

```js
JavaScript: 1.87
Rust: 203
```

#### 256 characters

```js
JavaScript: 1.17
Rust: 204
```

#### 512 characters

```js
JavaScript: 1.24
Rust: 204
```

#### 1 024 characters

```js
JavaScript: 1.20
Rust: 206
```

#### 2 048 characters

```js
JavaScript: 1.48
Rust: 201
```

#### 4 096 characters

```js
JavaScript: 2.37
Rust: 204
```

#### 8 192 characters

```js
JavaScript: 2.29
Rust: 203
```

#### 16 384 characters

```js
JavaScript: 4.43
Rust: 214
```

### Even bigger tokens

#### 100 000 characters

```js
JavaScript: 20
Rust: 221
```

#### 500 000 characters

```js
JavaScript: 92
Rust: 217
```

#### 1 000 000 characters

```js
JavaScript: 219
Rust: 246
```

#### 100 000 000 characters

```js
JavaScript: CRASH
Rust: 3621
```

#### 1 000 000 000 characters

```js
JavaScript: CRASH
Rust: 34985
```

#### 1 000 000 000 000 characters

```js
JavaScript: CRASH
Rust: CRASH
```

## Results

The results from the character generation are definitely shocking, I expected Rust to be faster in all of them, yet this was not the case. Rust did handle server side progressing better yet its standard response time is around 200ms.  
The latest tests (character generation) were performed using Insomnia.  

Rust does perform better for enterprise applications as we can see by the stress test.
