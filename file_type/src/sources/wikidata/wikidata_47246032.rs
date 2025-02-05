use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47246032: FileFormat = FileFormat {
    id: 47_246_032,
    source_type: SourceType::Wikidata,
    name: "PowerVR Object Data",
    extensions: &["pod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
