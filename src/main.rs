// fn largest_i32(list: &[i32]) -> &i32 {
//   let mut largest = &list[0];

//   for item in list {
//       if item > largest {
//           largest = item;
//       }
//   }

//   largest
// }

// fn largest_char(list: &[char]) -> &char {
//   let mut largest = &list[0];

//   for item in list {
//       if item > largest {
//           largest = item;
//       }
//   }

//   largest
// }

// fn main() {
//   let number_list = vec![34, 50, 25, 100, 65];

//   let result = largest_i32(&number_list);
//   println!("The largest number is {result}");

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest_char(&char_list);
//   println!("The largest char is {result}");
// }
///////////////////////Это ошибка использовать функцию с тем же кодом в двух местах///////////////////////////////

// fn largest<T>(list: &[T]) -> &T {
//   let mut largest = &list[0];

//   for item in list {
//       if item > largest {
//           largest = item;
//       }
//   }

//   largest
// }

// fn main() {
//   let number_list = vec![34, 50, 25, 100, 65];

//   let result = largest(&number_list);
//   println!("The largest number is {result}");

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest(&char_list);
//   println!("The largest char is {result}");
// }

///Это вот пример работы правльного кода. Он заработает если добавить в <T ... > ----> :std::cmp::PartialOrd /////////////////
 

// struct Point<T> {
//   x: T,
//   y: T,
// }

// fn main() {
//   let wont_work = Point { x: 5, y: 4.0 };
// }

/// Здесь код выдаёт ошибку так как y имеет тип float а x int /////////

// struct Point<T, U> {
//   x: T,
//   y: U,
// }

// fn main() {
//   let both_integer = Point { x: 5, y: 10 };
//   let both_float = Point { x: 1.0, y: 4.0 };
//   let integer_and_float = Point { x: 5, y: 4.0 };
// }

//// тут праильная реализация возможности делать разные типы ///////////////////

 
// struct Point<X1, Y1> {
//   x: X1,
//   y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//   fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//       Point {
//           x: self.x,
//           y: other.y,
//       }
//   }
// }

// fn main() {
//   let p1 = Point { x: 5, y: 10.4 };
//   let p2 = Point { x: "Hello", y: 'c' };

//   let p3 = p1.mixup(p2);

//   println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

/// Здесь мы use mixup он принимает 2 типа на вход и позволяет выводить 2 разных типа смешивая их /////////
 
 
 
 
///__________________________*ADD TWO NUMBERS*  https://leetcode.com/problems/add-two-numbers/ ____________




#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut new_node = ListNode::new(0);
      let mut current = &mut new_node;
      let mut carry = 0;

      while l1.is_some() || l2.is_some() || carry != 0
      {
        let sum = carry 
        + l1.as_ref().map_or(0, |node|node.val) 
        + l2.as_ref().map_or(0, |node|node.val);

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        l1 = l1.and_then(|node|node.next);
        l2 = l2.and_then(|node|node.next);

      }



      new_node.next 
}

fn main() {
  let l1 = Some(Box::new (ListNode{
    val: 2,
    next: Some(Box::new (ListNode{
      val: 4,
      next: Some(Box::new (ListNode::new(3)))
      }))
    }));
  

  let l2 = Some(Box::new (ListNode{
    val: 5,
    next: Some(Box::new (ListNode{
      val: 6,
      next: Some(Box::new (ListNode::new(4)))
      }))
    }));


    let result = add_two_numbers(l1, l2);

    // Выводим результат
    println!("{:?}", result);

  }

/// ________________S.o.l.u.t.i.o.n of ADD TWO NUMBERS___________________
