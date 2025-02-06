use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1068805: FileFormat = FileFormat {
    id: 1_068_805,
    source_type: SourceType::Wikidata,
    name: ".properties",
    extensions: &["properties"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
