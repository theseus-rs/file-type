use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771272: FileFormat = FileFormat {
    id: 28_771_272,
    source_type: SourceType::Wikidata,
    name: "MVG",
    extensions: &["mvg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
