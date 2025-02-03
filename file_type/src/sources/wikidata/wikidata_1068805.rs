use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1068805: FileFormat = FileFormat {
    id: 1_068_805,
    source_type: SourceType::Wikidata,
    name: ".properties",
    extensions: &["properties"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
