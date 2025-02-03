use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109623939: FileFormat = FileFormat {
    id: 109_623_939,
    source_type: SourceType::Wikidata,
    name: "WritePlus",
    extensions: &["stt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
