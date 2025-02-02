use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122407016: FileFormat = FileFormat {
    id: 122_407_016,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior CWP Project",
    extensions: &["cwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
