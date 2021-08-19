use gol_lib::Field;

pub struct Strategy {
    field: Field,
}

impl Strategy {
    pub fn new(field: Field) -> Self {
        Strategy { field }
    }
}

impl Iterator for Strategy {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        let mut field = self.field.clone();

        let mut updated_any = false;
        for x in 0..self.field.width() {
            for y in 0..self.field.height() {
                if let Some(value) = self.field.advance_one((x, y)) {
                    *field.value_mut((x, y)) = value;
                    updated_any = true;
                }
            }
        }

        if !updated_any {
            return None;
        }

        self.field = field.clone();

        Some(field)
    }
}
