use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110456179: FileFormat = FileFormat {
    id: 110_456_179,
    source_type: SourceType::Wikidata,
    name: "Standard Data Format",
    extensions: &["sdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
