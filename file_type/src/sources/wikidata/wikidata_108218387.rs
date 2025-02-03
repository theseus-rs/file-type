use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108218387: FileFormat = FileFormat {
    id: 108_218_387,
    source_type: SourceType::Wikidata,
    name: "Citation File Format",
    extensions: &["cff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
