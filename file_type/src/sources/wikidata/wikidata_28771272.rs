use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771272: FileFormat = FileFormat {
    id: 28_771_272,
    source_type: SourceType::Wikidata,
    name: "MVG",
    extensions: &["mvg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
