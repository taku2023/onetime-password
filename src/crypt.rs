use std::{fmt::Debug, num::Wrapping};

pub struct Block{
  words: [u32;16],
}

fn shift_n(word:u32,n:usize) -> u32{
  word << n | word >> (32-n)
}

impl Block{
  
  pub fn new(message:&str)->Vec<Block>{
    let message = message.bytes().collect::<Vec<u8>>();
    Block::from_bytes(message)
  }

  pub fn from_bytes(mut message: Vec<u8>) -> Vec<Block>{
    let len = message.len();
    //padding
    if message.len() % 64 !=0  {
      message.push(0x80); //b10000000 を追加
      while message.len() % 64 != 56 {
        message.push(0x00);
      }
      //padding last 64 bits as representation of length of message
      let len_as_bytes = (len * 8_usize) .to_be_bytes();
      message.append(&mut len_as_bytes.to_vec());
    }
    message.chunks(64).map(|vec|{
      let words = vec.chunks(4).map(|word|{
           u32::from_be_bytes(word.try_into().unwrap())
      }).collect::<Vec<u32>>();
      Block{ words: words.try_into().unwrap()}
    }).collect::<Vec<Block>>()
  }

}


const K:[u32;4] = [0x5A827999,0x6ED9EBA1,0x8F1BBCDC,0xCA62C1D6];

fn append(words:[u32;16]) -> [u32;80]{
  let mut appends = words.to_vec();
  for i in 16..80{
    appends.push(shift_n(appends[i-3] ^ appends[i-8] ^ appends[i-14] ^ appends[i-16],1));
  }
  appends.try_into().unwrap()
}

fn logical(b:u32,c:u32,d:u32,t:usize)->u32{
  match t / 20{
    0 =>{
      (b & c) | (!b & d)  
    },
    1 | 3 => {
      b ^ c ^ d
    },
    2 =>{
      (b & c) | (b & d) | (c & d)
    },
    _ => panic!("{} must between 0..80",t)
  }
}

pub fn digest(blocks:Vec<Block>)->[u8;20]{
  
  let mut h:[u32;5] = [0x67452301,0xEFCDAB89,0x98BADCFE,0x10325476,0xC3D2E1F0];

  for block in blocks{
    let words = append(block.words);
    let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
    let mut temp: Wrapping<u32>;
  
    for i in 0..words.len(){
      temp = Wrapping(shift_n(a, 5)) + Wrapping(logical(b, c, d, i)) + Wrapping(e) + Wrapping(words[i]) + Wrapping(K[i/20]);
      e = d;
      d = c;
      c = shift_n(b, 30);
      b = a;
      a = temp.0;
    }
    h[0] = h[0].wrapping_add(a);
    h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c);
    h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e);
  }
  h.iter().flat_map(|&x| x.to_be_bytes().to_vec()).collect::<Vec<u8>>().try_into().unwrap()
}


// Test for Block struct and new function
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_padding_block() {
        let message = "abcde";
        let blocks = Block::new(message);
        assert_eq!(blocks.len(),1);

        let block = &blocks[0];
        assert_eq!(format!("{:08X}",block.words[0]),"61626364");
        assert_eq!(format!("{:08X}",block.words[1]),"65800000");
        assert_eq!(format!("{:08X}",block.words[2]),"00000000");
        assert_eq!(format!("{:08X}",block.words[3]),"00000000");
        assert_eq!(format!("{:08X}",block.words[4]),"00000000");
        assert_eq!(format!("{:08X}",block.words[5]),"00000000");
        assert_eq!(format!("{:08X}",block.words[6]),"00000000");
        assert_eq!(format!("{:08X}",block.words[7]),"00000000");
        assert_eq!(format!("{:08X}",block.words[8]),"00000000");
        assert_eq!(format!("{:08X}",block.words[9]),"00000000");
        assert_eq!(format!("{:08X}",block.words[10]),"00000000");
        assert_eq!(format!("{:08X}",block.words[11]),"00000000");
        assert_eq!(format!("{:08X}",block.words[12]),"00000000");
        assert_eq!(format!("{:08X}",block.words[13]),"00000000");
        assert_eq!(format!("{:08X}",block.words[14]),"00000000");
        assert_eq!(format!("{:08X}",block.words[15]),"00000028");
    }

    #[test]
    fn test_sha1(){
      let message  = "apple";
      let blocks = Block::new(message);
      let digest = digest(blocks);
      let expect = digest.map(|byte|{
        format!("{:02X}",byte)
      }).join("");
      assert_eq!(expect,"d0be2dc421be4fcd0172e5afceea3970e2f3d940".to_uppercase());
    }

    #[test]
    fn test_sha1_2(){
      let message  = "hello world";
      let blocks = Block::new(message);
      let digest = digest(blocks);
      let expect = digest.map(|byte|{
        format!("{:02X}",byte)
      }).join("");
      assert_eq!(expect,"2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_uppercase());
    }

    #[test]
    fn test_sha1_3(){
      let message  = "This is not a test, this is j-alert. Everyone please run away immediately.";
      let blocks = Block::new(message);
      let digest = digest(blocks);
      let expect = digest.map(|byte|{
        format!("{:02X}",byte)
      }).join("");
      assert_eq!(expect,"700f7f82c981a500cda830d64f5e1743278a9f91".to_uppercase());
    }
}

