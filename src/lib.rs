//! Kuvastaa neljänsuora -peliä, jossa kaksi pelaaja yrittävät saada neljän suoran omia
//! nappuloitansa ruudukkoon pudottelemalla niitä päällekkäin.
mod ruudukko;
pub mod peli;

use peli::Pelaaja;
use peli::VuoropohjainenYhdenLuvunPeli;
use peli::Pelivirhe;
use peli::Tulos;
use ruudukko::Ruutu;


/// Neljänsuora peli, jossa kaksi pelaajaa yrittävät muodostaa ruudukkoon neljän oman värisen
/// nappulan suoran.
pub struct NSPeli{
    vuoro: Pelaaja,
    ruudukko: ruudukko::Peliruudukko,
    tilanne: Tulos,
}


/// Apufunktio voiton tarkistuksessa. Tarkistaa onko annettu ruutu voittava jne.
fn tarkista_ruutu(ruudun_vari: Option<Ruutu>, kerattava_vari: &mut Ruutu, j_pituus: &mut i8) -> Option<Tulos> {

    match ruudun_vari {
        None => *kerattava_vari = Ruutu::Tyhja,
        Some(Ruutu::Tyhja) => {*kerattava_vari = Ruutu::Tyhja;},
        Some(vari) => if vari == *kerattava_vari {
                *j_pituus += 1;
                if *j_pituus >= 4 {
                    match *kerattava_vari {
                        Ruutu::Punainen => return Some(Tulos::Voitto(Pelaaja::Musta)),
                        Ruutu::Sininen => return Some(Tulos::Voitto(Pelaaja::Valkea)),
                        _ => return None,
                    }
                }
            }
            else {
                *j_pituus = 1;
                *kerattava_vari = vari;
            },
    }
    None
}

impl NSPeli {
    /// Alustaa pelin seitsemällä sarakkeella ja kuudella rivillä, niin että valkea pelaaja
    /// aloittaa.
    pub fn aloita_peli() -> NSPeli {
        println!("Luodaan ruudukko");
        NSPeli {vuoro: Pelaaja::Valkea, ruudukko: ruudukko::luo_ruudukko(7,6), tilanne: Tulos::Kesken}
    }


    /// Tarkistaa onko annettu sarakkeen ylimmäinen nappula voittava nappula.
    /// # Arguments
    ///
    /// * `luku` - Sarake, jonka ylimmäinen nappula tarkistetaan.
    fn tarkista_voitto(&self, luku: i8) -> Tulos {
        let mut j_pituus = 0;
        let mut apu_vari = Ruutu::Tyhja;

        //Selvitetään nappula pinon korkeus.
        let mut nappulan_rivi = -1;
        for i in 0..self.ruudukko.anna_korkeus(){
            match self.ruudukko.anna_arvo_ruudussa(i, luku){
                None => break,
                Some(Ruutu::Tyhja) => break,
                _ => nappulan_rivi = nappulan_rivi + 1,
            }
        }

        //println!("{:?}", (luku, nappulan_rivi));

        for sarake in 0..self.ruudukko.anna_leveys() {
            let ruudun_vari = self.ruudukko.anna_arvo_ruudussa(nappulan_rivi, sarake);
            match tarkista_ruutu(ruudun_vari, &mut apu_vari, &mut j_pituus) {
                None => continue,
                Some(a) => return a,
            }
        }

        j_pituus = 0;
        apu_vari = Ruutu::Tyhja;

        for rivi in 0..self.ruudukko.anna_korkeus() {
            let ruudun_vari = self.ruudukko.anna_arvo_ruudussa(rivi, luku);
            match tarkista_ruutu(ruudun_vari, &mut apu_vari, &mut j_pituus) {
                None => continue,
                Some(a) => return a,
            }
        }

        j_pituus = 0;
        apu_vari = Ruutu::Tyhja;

        let (mut x,mut y) = (luku, nappulan_rivi);
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
        }

        let a = self.ruudukko.anna_leveys();
        let b = self.ruudukko.anna_korkeus();
            
        let min = if a < b {a} else {b};
        for i in 0..min {
            let ruudun_vari = self.ruudukko.anna_arvo_ruudussa(y+i, x+i);
            match tarkista_ruutu(ruudun_vari, &mut apu_vari, &mut j_pituus) {
                None => continue,
                Some(a) => return a,
            }
        }

        let (mut x,mut y) = (luku, nappulan_rivi);
        while x > 0 && y > 0 {
            x -= 1;
            y += 1;
        }

        j_pituus = 0;
        apu_vari = Ruutu::Tyhja;


        for i in 0..min {
            let ruudun_vari = self.ruudukko.anna_arvo_ruudussa( y-i, x+i);
            match tarkista_ruutu(ruudun_vari, &mut apu_vari, &mut j_pituus) {
                None => continue,
                Some(a) => return a,
            }
        }

        //Tarkistetaan onko tyhjiä ruutuja.
        for i in 0..self.ruudukko.anna_leveys() {
            match self.ruudukko.anna_arvo_ruudussa(self.ruudukko.anna_korkeus()-1, i) {
                Some(Ruutu::Tyhja) => return Tulos::Kesken,
                _ => (),
            }
        }

        Tulos::Tasapeli
    }
}


impl VuoropohjainenYhdenLuvunPeli for NSPeli{

    /// Kertoo kumman pelaan vuoro on.
    fn anna_vuoro(&self) -> Pelaaja{
        self.vuoro
    }

    /// Antaa pelin ohjeet tekstimuodossa.
    fn anna_ohjeet(&self) -> String{
        "Neljänsuora-pelissä pitää pudotella nappuloita ruudukkoon toistensa päälle ja siten pyrkiä saamaan neljä omanväristä nappulaa vierekkäin joko vaakasuunnassa, pystysuunnassa tai vinottain. Tässä ensiksi onnistunut pelaaja voittaa pelin. Nappuloita pudotellaan vuorotellen kuitenkin niin, että valkea aloittaa pelin.".to_string()
    }

    fn anna_tilanne(&self) -> Tulos{
        self.tilanne
    }


    /// Laittaa annettua lukua vastaavaan sarakkeeseen vuorossa olevan pelaajan värisen nappulan.
    ///
    /// # Arguments
    ///
    /// * `luku` - Sarake, johon nappula laitetaan.
    fn pelaa(&mut self, luku: i8) -> Result<Tulos, Pelivirhe>{
        if self.tilanne != Tulos::Kesken {
            return Ok(self.tilanne); 
        }

        if luku < 0 {
            return Err(Pelivirhe::LiianPieniSyote);
        } 
        if luku >= self.ruudukko.anna_leveys(){
            return Err(Pelivirhe::LiianSuuriSyote);
        }

        let asetettava = match self.vuoro {
            Pelaaja::Musta => Ruutu::Punainen,
            Pelaaja::Valkea => Ruutu::Sininen,
        };

        for i in 0.. {
            if match self.ruudukko.anna_arvo_ruudussa(i,luku) {
                None => return Err(Pelivirhe::LaitonSiirto("Ei mahdu.".to_string())),
                Some(Ruutu::Punainen) => false,
                Some(Ruutu::Sininen) => false,
                Some(Ruutu::Tyhja) => true 
            }{
                self.ruudukko.muuta_arvo_ruudussa(i, luku, asetettava);//TODO: Matchaus olisi kiva.
                break;
            }
        }
        self.vuoro = match self.vuoro {
            Pelaaja::Musta => Pelaaja::Valkea,
            Pelaaja::Valkea => Pelaaja::Musta,
        };

        self.tilanne = self.tarkista_voitto(luku);
        Ok(self.tilanne)
    }


    /// Esittää pelitilanteen tekstimuodossa.
    fn esita_peli_tekstina(&self) -> String{
        self.ruudukko.anna_ruudukko_tekstina()
    }
}
