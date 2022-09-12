#[allow(unused)]
use super::vek::Vek;

#[macro_export]
macro_rules! vek {
    ($($element:expr,)*) => {{
        const COUNT:usize = $crate::vek![@COUNT;$($element),*];
        #[allow(unused_mut)]
        let mut vek = Vek::with_capacity(COUNT);
        $(vek.push($element);)*
        vek
    }};

    ($($element:expr),*) => {{
         $crate::vek![$($element,)*]
    }};

    ($element:expr;$count:expr)=>{{
      #[allow(unused_mut)]
      let mut vek = Vek::with_capacity($count);
      vek
     }
    };

     ($start:expr => $end:expr) => {{
     let mut vek = Vek::with_capacity($end-$start);
     for num in $start..=$end{
        vek.push(num);
     }
      vek
     }};

    (@COUNT;$($element:expr),*)=>{
       <[()]>::len(&[$($crate::vek![@SUBST;$element]),*])
    };

    (@SUBST;$element:expr)=>{
    (())
    }
}
pub use vek;
