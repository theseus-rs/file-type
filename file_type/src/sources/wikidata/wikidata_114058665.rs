use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114058665: FileFormat = FileFormat {
    id: 114_058_665,
    source_type: SourceType::Wikidata,
    name: "Canon SIF File",
    extensions: &["sif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
