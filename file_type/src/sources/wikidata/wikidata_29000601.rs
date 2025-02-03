use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000601: FileFormat = FileFormat {
    id: 29_000_601,
    source_type: SourceType::Wikidata,
    name: "Patch Storage File",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
