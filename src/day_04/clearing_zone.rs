pub struct ClearingZone {
    start: u32,
    end: u32
}

impl ClearingZone {
    pub fn new(start: u32, end: u32) -> ClearingZone {
        ClearingZone { start, end }
    }
    pub fn includes(&self, other: &ClearingZone) -> bool {
        other.start >= self.start && other.end <= self.end 
    }
    pub fn includes_two_way(&self, other: &ClearingZone) -> bool {
        self.includes(other) || other.includes(self)
    }
    pub fn overlaps(&self, other: &ClearingZone) -> bool {
        self.includes_two_way(other)
        || (self.start >= other.start && self.start <= other.end)
        || (self.end >= other.start && self.end <= other.end)
    }
}

impl FromIterator<u32> for ClearingZone {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut into = iter.into_iter();
        ClearingZone::new(into.next().unwrap(), into.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::ClearingZone;

    #[test]
    fn test_includes() {
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(8, 12);
        assert!(zone_a.includes(&zone_b));
        assert!(!zone_b.includes(&zone_a));
    }
    #[test]
    fn test_includes_two_way() {
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(8, 12);
        assert!(zone_a.includes_two_way(&zone_b));
        assert!(zone_b.includes_two_way(&zone_a));
    }
    #[test]
    fn test_overlaps() {
        // fully included
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(8, 12);
        assert!(zone_a.overlaps(&zone_b));
        assert!(zone_b.overlaps(&zone_a));
        // edge overlaps end
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(15, 16);
        assert!(zone_a.overlaps(&zone_b));
        assert!(zone_b.overlaps(&zone_a));
        // overlaps end
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(9, 16);
        assert!(zone_a.overlaps(&zone_b));
        assert!(zone_b.overlaps(&zone_a));
        // edge overlaps start
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(2, 5);
        assert!(zone_a.overlaps(&zone_b));
        assert!(zone_b.overlaps(&zone_a));
        // overlaps start
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(2, 6);
        assert!(zone_a.overlaps(&zone_b));
        assert!(zone_b.overlaps(&zone_a));
        // no overlap start
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(2, 4);
        assert!(!zone_a.overlaps(&zone_b));
        assert!(!zone_b.overlaps(&zone_a));
        // no overlap end
        let zone_a = ClearingZone::new(5, 15);
        let zone_b = ClearingZone::new(16, 18);
        assert!(!zone_a.overlaps(&zone_b));
        assert!(!zone_b.overlaps(&zone_a));
    }
}