pub struct Allergies {
    score: u32,
}
#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    const ALLERGENS: [Allergen; 8] = [
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    pub fn new(score: u32) -> Self {
        return Allergies { score };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return self.allergies().contains(allergen);
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return Self::ALLERGENS
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| self.score & (1 << i) != 0)
            .map(|(_, allergen)| allergen)
            .collect();
    }
}
