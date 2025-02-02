use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207427: FileFormat = FileFormat {
    id: 28_207_427,
    source_type: SourceType::Wikidata,
    name: "Verity Image Format",
    extensions: &["vif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
