use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59616045: FileFormat = FileFormat {
    id: 59_616_045,
    source_type: SourceType::Wikidata,
    name: "Zope export file",
    extensions: &["zexp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
