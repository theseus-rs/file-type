use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119494021: FileFormat = FileFormat {
    id: 119_494_021,
    source_type: SourceType::Wikidata,
    name: "SnagIt Capture File",
    extensions: &["snag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
