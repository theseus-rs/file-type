use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111430980: FileFormat = FileFormat {
    id: 111_430_980,
    source_type: SourceType::Wikidata,
    name: "ExtendScript Script File",
    extensions: &["jxs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
