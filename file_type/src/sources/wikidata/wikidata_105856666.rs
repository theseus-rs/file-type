use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856666: FileFormat = FileFormat {
    id: 105_856_666,
    source_type: SourceType::Wikidata,
    name: "Unreal Engine Project",
    extensions: &["uproject"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
