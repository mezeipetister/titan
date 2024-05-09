use serde::{Deserialize, Serialize};

struct Object {
    id: String,
    commit_id: String,
    version: i32,
    data_json: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Frame {
    Single {
        bytes: Vec<u8>,
    },
    Chunk {
        bytes: Vec<u8>,
        index: usize,
        total: usize,
    },
}

fn bytes_to_frames(bytes: Vec<u8>, max_size_bytes: usize) -> Vec<Frame> {
    if bytes.len() <= max_size_bytes {
        return vec![Frame::Single { bytes }];
    } else {
        let mut frames = vec![];
        let mut index = 0;
        let mut total = bytes.len();
        while total > 0 {
            let chunk_size = std::cmp::min(max_size_bytes, total);
            let chunk = bytes[index..index + chunk_size].to_vec();
            frames.push(Frame::Chunk {
                bytes: chunk,
                index,
                total,
            });
            index += chunk_size;
            total -= chunk_size;
        }
        return frames;
    }
}

fn frames_to_bytes(frames: Vec<Frame>) -> Vec<u8> {
    let mut _bytes = vec![];
    for frame in frames {
        match frame {
            Frame::Single { bytes } => bytes.iter().for_each(|b| _bytes.push(*b)),
            Frame::Chunk { bytes, .. } => bytes.iter().for_each(|b| _bytes.push(*b)),
        }
    }
    return _bytes;
}
