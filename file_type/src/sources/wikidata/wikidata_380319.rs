use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_380319: FileFormat = FileFormat {
    id: 380_319,
    source_type: SourceType::Wikidata,
    name: "dynamic-link library",
    extensions: &["dll"],
    media_types: &[
        "application/vnd.microsoft.portable-executable",
        "application/x-msdownload",
    ],
    signatures: &[],
    related_formats: &[],
};
