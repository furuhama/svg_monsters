#[derive(Debug)]
pub struct MonsterPattern {
    leg: Vec<Vec<usize>>,
    hair: Vec<Vec<usize>>,
    arm: Vec<Vec<usize>>,
    body: Vec<Vec<usize>>,
    eye: Vec<Vec<usize>>,
    mouth: Vec<Vec<usize>>,
}

impl MonsterPattern {
    fn new() -> Self {
        Self {
            leg: Vec::<Vec<usize>>::new(),
            hair: Vec::<Vec<usize>>::new(),
            arm: Vec::<Vec<usize>>::new(),
            body: Vec::<Vec<usize>>::new(),
            eye: Vec::<Vec<usize>>::new(),
            mouth: Vec::<Vec<usize>>::new(),
        }
    }

    pub fn generate(&self) -> Vec<Vec<usize>> {
        let mut shape = self.unite_shape();
        let face_parts = self.unite_face_parts();

        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if face_parts[y][x] == 1 {
                    shape[y][x] = 0;
                }
            }
        }

        shape
    }

    fn unite_shape(&self) -> Vec<Vec<usize>> {
        let mut shape = union(&self.leg, &self.hair);
        shape = union(&shape, &self.arm);
        shape = union(&shape, &self.body);

        shape
    }

    fn unite_face_parts(&self) -> Vec<Vec<usize>> {
        union(&self.eye, &self.mouth)
    }

    fn set_leg(&mut self, leg: Vec<Vec<usize>>) {
        self.leg = leg;
    }

    fn set_hair(&mut self, hair: Vec<Vec<usize>>) {
        self.hair = hair;
    }

    fn set_arm(&mut self, arm: Vec<Vec<usize>>) {
        self.arm = arm;
    }

    fn set_body(&mut self, body: Vec<Vec<usize>>) {
        self.body = body;
    }

    fn set_eye(&mut self, eye: Vec<Vec<usize>>) {
        self.eye = eye;
    }

    fn set_mouth(&mut self, mouth: Vec<Vec<usize>>) {
        self.mouth = mouth;
    }
}

fn union(one: &Vec<Vec<usize>>, other: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = Vec::<Vec<usize>>::new();

    for y in 0..one.len() {
        let mut result_y = Vec::<usize>::new();

        for x in 0..one[y].len() {
            if one[y][x] == 0 && other[y][x] == 0 {
                result_y.push(0);
            } else {
                result_y.push(1);
            }
        }

        result.push(result_y);
    }

    result
}

mod test {
    use super::{MonsterPattern, union};

    #[test]
    fn monster_pattern() {
        let mut monster_pattern = MonsterPattern::new();

        let leg = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0]
        ];
        let hair = vec![
            vec![1, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0]
        ];
        let arm = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0]
        ];
        let body = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ];
        monster_pattern.set_leg(leg);
        monster_pattern.set_hair(hair);
        monster_pattern.set_arm(arm);
        monster_pattern.set_body(body);

        let shape = vec![
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0]
        ];
        assert_eq!(monster_pattern.unite_shape(), shape);

        let eye = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0]
        ];
        let mouth = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ];
        monster_pattern.set_eye(eye);
        monster_pattern.set_mouth(mouth);

        let face_parts = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ];
        assert_eq!(monster_pattern.unite_face_parts(), face_parts);

        let monster = vec![
            vec![1, 0, 0, 1],
            vec![1, 0, 1, 1],
            vec![0, 1, 0, 0],
            vec![0, 1, 1, 0]
        ];
        assert_eq!(monster_pattern.generate(), monster);
    }

    #[test]
    fn test_union() {
        let vec1 = vec![
            vec![0, 0],
            vec![1, 1]
        ];
        let vec2 = vec![
            vec![0, 1],
            vec![0, 1]
        ];

        let result = vec![
            vec![0, 1],
            vec![1, 1]
        ];

        assert_eq!(union(&vec1, &vec2), result);
    }
}
