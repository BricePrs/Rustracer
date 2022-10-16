# Rustracer
A basic raytracer implementation(on CPU) I'm currently working on in order to learn Rust

## Current state

This raytracer can only handles spheres and three types of `material`: 
 + `Lambertian`
 + `Dielectric`
 + `Metalic`

It also support `depth of field`.

While computing the output image, the current state is displayed in a window managed on a different thread.

## 
![render3](https://user-images.githubusercontent.com/44588205/196061050-c8dc683d-22be-47bb-9d54-bcd1b02bc8ff.png)
![render4](https://user-images.githubusercontent.com/44588205/196061053-72c9979a-5f5f-4cf3-87df-f4bb6e2a2fdb.png)
![render5](https://user-images.githubusercontent.com/44588205/196061056-ec4dc997-54fe-4e84-bac8-ec8ad85b6157.png)
