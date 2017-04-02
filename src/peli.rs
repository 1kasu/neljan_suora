//! Sisältää traitin vuoropohjaisille peleille, jotka toimivat yhden annetun luvun perusteella.

/// Traitti vuoropohjaiselle pelille, jossa annetaan aina yksi luku pelille, jonka mukaan
/// osataan toimia.
pub trait VuoropohjainenYhdenLuvunPeli{
    /// Antaa tiedon siitä, että kumman pelaajan vuoro on menossa.
    fn anna_vuoro(&self) -> Pelaaja;
    /// Antaa pelin tekstimuotoisen ohjeen.
    fn anna_ohjeet(&self) -> String;
    /// Muuttaa pelin tilaa annetulla luvulla.
    fn pelaa(&mut self, i8) -> Result<Tulos, Pelivirhe>;
    /// Esittää pelitilanteen tekstimuodossa.
    fn esita_peli_tekstina(&self) -> String;
    /// Antaa nykyisen pelitilanteen
    fn anna_tilanne(&self) -> Tulos;
}

/// Kahden pelattavissa peleissä pelaajan väri oletuksella, että aloittaja on valkea aina
#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub enum Pelaaja{
    /// Musta pelaaja
    Musta,
    /// Valkea pelaaja
    Valkea,
}


/// Kuvastaa pelissä tapahtunutta syötteestä johtunutta virhettä.
pub enum Pelivirhe {
    /// Annettu syöte on liian suuri eli suurempi kuin ruudukon leveys.
    LiianPieniSyote,
    /// Annettu syöte on liian pieni eli pienempi kuin nolla.
    LiianSuuriSyote,
    /// Annettu syöte on laiton. Sisältää perustelun miksi.
    LaitonSiirto(String),
}


/// Kertoo pelin tuloksen.
#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub enum Tulos {
    /// Peli päättyi voittoon. Sisältää tiedon voittaneesta pelaajasta.
    Voitto(Pelaaja),
    /// Peli on yhä kesken.
    Kesken,
    /// Peli päättyi tasapeliin.
    Tasapeli,
}
