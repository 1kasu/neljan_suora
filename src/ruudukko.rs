//! Sisältää yleisesti käytettävän ruudukko logiikan.

/// Kuvastaa peliruudukkoa, jolla on ominaisuutena leveys, korkeus ja itse varsinainen
/// ruudukko.
pub struct Peliruudukko{
    leveys: i8,
    korkeus: i8,
    ruudukko: Vec<Vec<Ruutu>>,
}

static TYHJA:char = ' ';
static EROTIN:char = '|';
static SININEN:char = 'X';
static PUNAINEN:char = 'O';

impl Peliruudukko {
    /// Palauttaa ruudukon leveyden.
    pub fn anna_leveys(&self) -> i8 { self.leveys }
    /// Palauttaa ruudukon korkeuden.
    pub fn anna_korkeus(&self) -> i8 { self.korkeus }

    /// Antaa haettavan ruudukon tilan.
    ///
    /// # Arguments
    ///
    /// * `leveys` - Sijainti y-akselilla, alkaa nollasta.
    /// * `korkeus` - Sijainti x-akselilla, alkaa nollasta.
    pub fn anna_arvo_ruudussa(&self, leveys: i8, korkeus: i8) -> Option<Ruutu> {
        match self.ruudukko.get(leveys as usize).map(|x| x.get(korkeus as usize)) {
            Some(a) => {
                match a {
                    None => None,
                    Some(c) => Some(*c),
                }
            },
            None => None,
        }
    }


    /// Muuttaa annetun ruudun tilan halutuksi.
    ///
    /// # Arguments
    ///
    /// * `leveys` - Sijainti y-akselilla, alkaa nollasta.
    /// * `korkeus` - Sijainti x-akselilla, alkaa nollasta.
    pub fn muuta_arvo_ruudussa(&mut self, leveys: i8, korkeus: i8, asetettava: Ruutu) -> bool {
        match self.ruudukko.get_mut(leveys as usize).map(|x| x.get_mut(korkeus as usize)) {
            Some(a) => {
                match a {
                    None => false,
                    Some(c) => {
                        *c = asetettava;
                        true
                    },
                }
            },
            None => false,
        }
    }
    

    /// Antaa ruudukon tekstimuodossa.
    pub fn anna_ruudukko_tekstina(&self) -> String{
        let mut teksti: String = String::from("");
        for rivi in self.ruudukko.iter().rev() {
            teksti.push(EROTIN);
            for ruutu in rivi.iter() {
                match *ruutu {
                    Ruutu::Tyhja => teksti.push(TYHJA),
                    Ruutu::Punainen => teksti.push(PUNAINEN),
                    Ruutu::Sininen => teksti.push(SININEN),
                }
                teksti.push(EROTIN);
            }
            teksti.push('\n');
        }
        teksti
    }
}

/// Kuvastaa ruudun tilaa, joka voi olla tyhjä tai sisältää joko punaisen tai sinisen nappulan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ruutu{
    /// Tyhjä ruutu
    Tyhja,
    /// Ruudussa on punainen nappula
    Punainen, 
    /// Ruudussa on sininen nappula
    Sininen,
}


/// Luo ruudukon
///
/// # Arguments
///
/// * `leveys` - Ruudukon leveys.
/// * `korkeus` - Ruudukolle annettava korkeus.
pub fn luo_ruudukko(leveys: i8, korkeus: i8) -> Peliruudukko{
    let mut ruudukko: Vec<Vec<Ruutu>> = vec![];

    let mut i = 0;
    while i < korkeus{
        let mut j = 0;
        let mut rivi: Vec<Ruutu> = vec![];
        while j < leveys {
            rivi.push(Ruutu::Tyhja);
            j = j + 1;
        }
        ruudukko.push(rivi);
        i = i + 1;
    }

    Peliruudukko {leveys: leveys, korkeus: korkeus, ruudukko: ruudukko}
}


#[test]
fn testi(){
    let ruudukko = luo_ruudukko(4,5);
    assert_eq![ruudukko.leveys, 4];
    assert_eq![ruudukko.korkeus, 5];
}
