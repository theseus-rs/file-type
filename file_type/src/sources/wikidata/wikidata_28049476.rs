use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049476: FileFormat = FileFormat {
    id: 28_049_476,
    source_type: SourceType::Wikidata,
    name: "RGB Intermediate Format",
    extensions: &["rgb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
