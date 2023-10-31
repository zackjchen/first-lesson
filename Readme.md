# Second course



## Lifttime

1. 定一个个变量的时候，声明周期产生，并起作用，一直到变量离开drop的时候结束, 大部分情况下生命周期将与作用域重合

   ```rust
   // 一般来说在
   {
   	let a = 1;
     // 在}前结束生命周期
   }
   ```

   

2. 借用和引用小于的声明周期小于等于拥有所有权的这个变量

   ```rust
   //变量失效后再继续引用是非法的
   {
     {
       let a = 1;
       let b = &a;
     }
     println!("{b}")
   }
   ```

   

3. 生命周期一般保持最短, 借用检查器总是试图最小化生命周期的范围

   ```rust
   let x = 0;
   let y = &x;
   let z = &y;
   --------------------------------------------------------------------
   // NOTE: `'a: {` 和 `&'b x` 不是有效的语法，这里只是为了说明 lifetime 的概念
   'a: {
       let x: i32 = 0;
       'b: {
           // y 的生命周期为 'b，因为这已经足够好
           let y: &'b i32 = &'b x;
           'c: {
               // 'c 同上所示
               let z: &'c &'b i32 = &'c y; // "a reference to a reference to an i32" (with lifetimes annotated)
           }
       }
   }
   ```

   

4. 若实现了Drop trait生命周期会延长

   https://nomicon.purewhite.io/dropck.html

   ```rust
   #[derive(Debug)]
   struct X<'a>(&'a i32);
   
   impl Drop for X<'_> {
       fn drop(&mut self) {}
   }
   fn main(){
     let mut data = vec![1, 2, 3];
     let x = X(&data[0]);
     println!("{:?}", x);
     data.push(4);
     // 编译器会在这里自动插入 drop 函数，也就意味着我们会访问 x 中引用的变量，因此编译失败
   }
   ```

   

5. 如果有多个输入生命周期位置，但其中一个是`&self`或`&mut self`，那么`self`的生命周期将被分配给*所有*被省略的输出生命周期

6. 不安全的代码经常会凭空产生引用或生命周期

   https://nomicon.purewhite.io/unbounded-lifetimes.html

   ```rust
   fn main() {
       // level_one::fun_a_to_Z();
       // level_two::fun_A_to_z();
       
       let dangling = get_str();
       println!("Invalid str: {}", dangling); // Invalid str: gӚ_`
   }
   fn get_str<'a>() -> &'a str {
       let a = &"abc".to_string() as *const String;
       unsafe { &*a }
   }
   ```

   

7. 

