use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207427: FileFormat = FileFormat {
    id: 28_207_427,
    source_type: SourceType::Wikidata,
    name: "Verity Image Format",
    extensions: &["vif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
