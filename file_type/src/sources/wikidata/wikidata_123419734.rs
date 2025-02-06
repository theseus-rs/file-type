use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123419734: FileFormat = FileFormat {
    id: 123_419_734,
    source_type: SourceType::Wikidata,
    name: "StuffIt Zip Archive",
    extensions: &["zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
