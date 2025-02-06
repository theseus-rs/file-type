use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770341: FileFormat = FileFormat {
    id: 28_770_341,
    source_type: SourceType::Wikidata,
    name: "M2k",
    extensions: &["m2k"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
