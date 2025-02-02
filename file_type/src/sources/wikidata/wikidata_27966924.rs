use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966924: FileFormat = FileFormat {
    id: 27_966_924,
    source_type: SourceType::Wikidata,
    name: "PSF1",
    extensions: &["minipsf", "minipsf1", "psf", "psf1", "psf1lib", "psflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
