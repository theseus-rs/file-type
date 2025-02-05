use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4545411: FileFormat = FileFormat {
    id: 4_545_411,
    source_type: SourceType::Wikidata,
    name: "Blizzard Game Picture",
    extensions: &["blp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
