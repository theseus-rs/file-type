use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125148800: FileFormat = FileFormat {
    id: 125_148_800,
    source_type: SourceType::Wikidata,
    name: "YAM Users",
    extensions: &["users"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
