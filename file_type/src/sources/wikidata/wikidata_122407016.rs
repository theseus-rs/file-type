use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122407016: FileFormat = FileFormat {
    id: 122_407_016,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior CWP Project",
    extensions: &["cwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
