use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59653966: FileFormat = FileFormat {
    id: 59_653_966,
    source_type: SourceType::Wikidata,
    name: "MySQL Table Definition Format",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
