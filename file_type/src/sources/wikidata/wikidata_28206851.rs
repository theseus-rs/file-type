use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206851: FileFormat = FileFormat {
    id: 28_206_851,
    source_type: SourceType::Wikidata,
    name: "Secure Pick Ax file",
    extensions: &["pax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
