use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109623939: FileFormat = FileFormat {
    id: 109_623_939,
    source_type: SourceType::Wikidata,
    name: "WritePlus",
    extensions: &["stt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
