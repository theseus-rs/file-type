use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206272: FileFormat = FileFormat {
    id: 28_206_272,
    source_type: SourceType::Wikidata,
    name: "HTC splashscreen",
    extensions: &["img", "nb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
