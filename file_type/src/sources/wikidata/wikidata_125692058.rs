use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692058: FileFormat = FileFormat {
    id: 125_692_058,
    source_type: SourceType::Wikidata,
    name: "OpenDocument HTML Template file",
    extensions: &["oth"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
