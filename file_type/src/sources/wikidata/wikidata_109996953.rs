use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109996953: FileFormat = FileFormat {
    id: 109_996_953,
    source_type: SourceType::Wikidata,
    name: "Autocad DMP File",
    extensions: &["dmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
