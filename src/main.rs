use std::cmp::{max, Reverse};

#[derive(Debug, Clone)]
struct Photo {
    id: String,
    width: u32,
    height: u32,
}

impl Photo {
    fn new(id: String, width: u32, height: u32) -> Self {
        Photo { id, width, height }
    }
}

struct Canvas {
    width: u32,
    photos: Vec<Photo>,
    layout: Vec<Vec<Option<Photo>>>,
}

impl Canvas {
  fn new(width: u32, photos: Vec<Photo>) -> Self {
      Canvas {
          width,
          photos,
          layout: Vec::new(),
      }
  }

  fn pack_photos(&mut self) {
      self.photos.sort_by_key(|p| Reverse((p.height, p.width)));

      for photo in &self.photos {
          let (pos_x, pos_y) = self.find_position_for_photo(photo.width);
          self.place_photo(photo.clone(), pos_x, pos_y);
      }
  }

  fn find_position_for_photo(&self, photo_width: u32) -> (usize, usize) {
  // Clone the heap because we need to iterate over it to find a spot wide enough for the photo.
  let mut heap = self.columns_heap.clone();

  while let Some(column) = heap.pop() {
      // Check if the photo can fit in the space starting from `column.x`
      let end_x = column.x + photo_width as usize;
      if end_x <= self.width as usize && self.can_place_photo_at_column(&column, photo_width) {
          return (column.x, column.height);
      }
  }

  // If we found no spot, we can place the photo at the end of the current lowest column.
  let lowest_column = self.columns_heap.peek().cloned().unwrap_or(Column { x: 0, height: 0 });
  (lowest_column.x, lowest_column.height)
  }

  // Helper function to check if a photo can be placed at a given column
  fn can_place_photo_at_column(&self, column: &Column, photo_width: u32) -> bool {
      let end_x = column.x + photo_width as usize;
      for x in column.x..end_x {
          if let Some(taller_column) = self.columns_heap.iter().find(|&c| c.x == x && c.height > column.height) {
              return false;
          }
      }
      true
  }

  fn place_photo(&mut self, photo: Photo) -> Result<(), String> {
      for pos_y in 0..self.layout.len() + 1 {
          for pos_x in 0..self.width as usize {
              if self.can_place_photo(&photo, pos_x, pos_y) {
                  for y in pos_y..pos_y + photo.height as usize {
                      for x in pos_x..min(pos_x + photo.width as usize, self.width as usize) {
                          self.layout[y][x] = Some(photo.clone());
                      }
                  }
                  return Ok(());
              }
          }
      }
  
      Err(format!("Cannot place photo '{}', not enough space.", photo.id))
  }
  
  // Helper function to determine if a photo can be placed at a given position
  fn can_place_photo(&self, photo: &Photo, pos_x: usize, pos_y: usize) -> bool {
      // Check if the photo will fit horizontally within the canvas width
      if pos_x + photo.width as usize > self.width as usize {
          return false;
      }
  
      // Ensure the photo will not overlap vertically with existing photos
      for y in pos_y..pos_y + photo.height as usize {
          if y >= self.layout.len() {
              continue;
          }
  
          for x in pos_x..pos_x + photo.width as usize {
              if let Some(_) = self.layout[y].get(x).and_then(|cell| cell.as_ref()) {
                  return false;
              }
          }
      }
  
      // No overlaps, the photo can be placed here
      true
  }


}

fn main() {
    let photos = vec![
        Photo::new("photo1".to_string(), 450, 35),
        Photo::new("photo2".to_string(), 500, 450),
        // etc. 
    ];

    let mut canvas = Canvas::new(1500, photos);
    canvas.pack_photos();

    for row in canvas.layout {
        for spot in row {
            match spot {
                Some(photo) => print!("{} ", photo.id),
                None => print!("--- "),
            }
        }
        println!("");
    }
}
