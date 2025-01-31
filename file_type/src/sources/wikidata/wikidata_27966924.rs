use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966924: FileFormat = FileFormat {
    id: 27_966_924,
    puid: "wikidata/27966924",
    name: "PSF1",
    extensions: &["minipsf", "minipsf1", "psf", "psf1", "psf1lib", "psflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
