use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113354835: FileFormat = FileFormat {
    id: 113_354_835,
    source_type: SourceType::Wikidata,
    name: "Snagit Preset file",
    extensions: &["snagpresets"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
