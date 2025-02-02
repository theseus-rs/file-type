use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966927: FileFormat = FileFormat {
    id: 27_966_927,
    source_type: SourceType::Wikidata,
    name: "PSF2",
    extensions: &["minipsf2", "psf2", "psf2lib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
