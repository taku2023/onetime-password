
struct Block{
  words: [u32;16],
}

fn shift_n(word:u32,n:usize) -> u32{
  word << n | word >> (32-n)
}

impl Block{
  
  fn new(message:&str)->Vec<Block>{
    let mut message = message.bytes().collect::<Vec<u8>>();
    let len = message.len();
    //padding
    if message.len() % 64 !=0  {
      message.push(0x80); //b10000000 を追加
      while message.len() % 56!= 0 {
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


mod processor{
  use super::{Block,shift_n};
  use std::num::Wrapping;

  const H:[u32;5] = [0x67452301,0xEFCDAB89,0x98BADCFE,0x10325476,0xC3D2E1F0];
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

  pub fn digest(message:&str)->String{
    let blocks = Block::new(message);
    
    let mut a = H[0];
    let mut b = H[1];
    let mut c = H[2];
    let mut d = H[3];
    let mut e = H[4];
    let mut temp:Wrapping<u32>;

    for block in blocks{
      let words = append(block.words);
      for i in 0..words.len(){
        temp = Wrapping(shift_n(a, 5)) + Wrapping(logical(b, c, d, i)) + Wrapping(e) + Wrapping(words[i]) + Wrapping(K[i/20]);
        e = d;
        d = c;
        c = shift_n(b, 30);
        b = a;
        a = temp.0;
      } 
    }
    let mut digest  = H[0].wrapping_add(a).to_be_bytes().map(|byte|{
      format!("{:02X}",byte)
    }).join("");
    let h1 = H[1].wrapping_add(b).to_be_bytes().map(|byte|{
      format!("{:02X}",byte)
    }).join("");
    let h2 = H[2].wrapping_add(c).to_be_bytes().map(|byte|{
      format!("{:02X}",byte)
    }).join("");
    let h3 = H[3].wrapping_add(d).to_be_bytes().map(|byte|{
      format!("{:02X}",byte)
    }).join("");
    let h4 = H[4].wrapping_add(e).to_be_bytes().map(|byte|{
      format!("{:02X}",byte)
    }).join("");
    digest.push_str(&h1);
    digest.push_str(&h2);
    digest.push_str(&h3);
    digest.push_str(&h4);
    digest
  }
}

// Test for Block struct and new function
#[cfg(test)]
mod tests {
    use self::processor::digest;

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
      assert_eq!(digest(message),"d0be2dc421be4fcd0172e5afceea3970e2f3d940".to_uppercase());
    }
}

