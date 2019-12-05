#![allow(dead_code)]

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use glam::*;

pub enum Face {
    Tri ([[usize; 3]; 3]),
    // Quad([[usize; 4]; 4]), triangles only who cares
}

pub struct OBJ {
    pub vertices: Vec<Vec3>,
    pub normals:  Vec<Vec3>,
    pub uvs:      Vec<Vec2>,
    pub faces:    Vec<Face>,
}

impl OBJ {
    pub fn iter<'a>(&'a self) -> OBJIter<'a> {
        OBJIter::<'a> {
            obj: self,
            index: 0,
            face_index: 0,
        }
    }
    pub fn from_file(filename: &str) -> std::io::Result<OBJ> {
        let mut obj = OBJ {
            vertices: Vec::new(),
            normals:  Vec::new(),
            uvs:      Vec::new(),
            faces:    Vec::new(),
        };
        
        let file = File::open(Path::new(filename))?;
        
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut iter = line.split_whitespace();
                match iter.next() {
                    Some("v") => {
                        let x = f32::from_str(iter.next().unwrap()).unwrap();
                        let y = f32::from_str(iter.next().unwrap()).unwrap();
                        let z = f32::from_str(iter.next().unwrap()).unwrap();
                        obj.vertices.push(Vec3::new(x, y, z));
                    },
                    Some("vn") => {
                        let x = f32::from_str(iter.next().unwrap()).unwrap();
                        let y = f32::from_str(iter.next().unwrap()).unwrap();
                        let z = f32::from_str(iter.next().unwrap()).unwrap();
                        obj.normals.push(Vec3::new(x, y, z));
                    },
                    Some("vt") => {
                        let x = f32::from_str(iter.next().unwrap()).unwrap();
                        let y = f32::from_str(iter.next().unwrap()).unwrap();
                        obj.uvs.push(Vec2::new(x, y));
                    }
                    
                    Some("f") => {
                        obj.faces.push(parse_face(&mut iter));
                    },
                    
                    //Skip unhandled stuff
                    None => continue,
                    _    => continue,
                }
            }
        }
        
        Ok(obj)
    }
}

fn parse_face<'a, I: Iterator<Item=&'a str>>(words: &mut I) -> Face {
    let mut face = [[0; 3]; 3];
    words.enumerate().for_each(|(i, word)| {
        let iter = word.split('/');
        let ints: Vec<usize> = iter.map(|s| usize::from_str(s).unwrap_or(0)).collect();
        for (j, int) in ints.iter().enumerate() {
            face[i][j] = *int;
        }
    });
    
    Face::Tri(face)
}

pub struct OBJIter<'a> {
    obj:   &'a OBJ,
    index:      usize,
    face_index: usize,
}
impl<'a> Iterator for OBJIter<'a> {
    type Item = (Vec3, Vec3, Vec2);
    
    fn next(&mut self) -> Option<Self::Item> {
        let obj = self.obj;
        //TODO check current face type
        if self.face_index >= 3 {
            self.face_index = 0;
            self.index += 1;
        }
        if let Some(face) = obj.faces.get(self.index) {
            match face {
                Face::Tri(ints) => {
                    let index = ints[self.face_index];
                    self.face_index += 1;
                    
                    Some((
                        obj.vertices[index[0] - 1],
                        
                        if index[2] > 0 {
                            obj.normals[index[2] - 1]
                        } else {
                            Vec3::new(0., 0., 0.)
                        },
                        
                        if index[1] > 0 {
                            obj.uvs[index[1] - 1]
                        } else {
                            Vec2::new(0., 0.)
                        }
                    ))
                }
            }
        } else {
            None
        }
    }
}
