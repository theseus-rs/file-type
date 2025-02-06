use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966924: FileFormat = FileFormat {
    id: 27_966_924,
    source_type: SourceType::Wikidata,
    name: "PSF1",
    extensions: &["minipsf", "minipsf1", "psf", "psf1", "psf1lib", "psflib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
