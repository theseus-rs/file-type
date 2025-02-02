use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856666: FileFormat = FileFormat {
    id: 105_856_666,
    source_type: SourceType::Wikidata,
    name: "Unreal Engine Project",
    extensions: &["uproject"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
