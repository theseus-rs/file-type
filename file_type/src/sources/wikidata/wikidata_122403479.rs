use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122403479: FileFormat = FileFormat {
    id: 122_403_479,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Resource File",
    extensions: &["rsrc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
