use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000601: FileFormat = FileFormat {
    id: 29_000_601,
    source_type: SourceType::Wikidata,
    name: "Patch Storage File",
    extensions: &["psf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
