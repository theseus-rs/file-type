use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966927: FileFormat = FileFormat {
    id: 27_966_927,
    source_type: SourceType::Wikidata,
    name: "PSF2",
    extensions: &["minipsf2", "psf2", "psf2lib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
