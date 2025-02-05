use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116331429: FileFormat = FileFormat {
    id: 116_331_429,
    source_type: SourceType::Wikidata,
    name: "Lawyer File",
    extensions: &["flp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
