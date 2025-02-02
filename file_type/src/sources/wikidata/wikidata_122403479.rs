use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122403479: FileFormat = FileFormat {
    id: 122_403_479,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Resource File",
    extensions: &["rsrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
