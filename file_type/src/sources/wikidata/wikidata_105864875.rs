use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864875: FileFormat = FileFormat {
    id: 105_864_875,
    source_type: SourceType::Wikidata,
    name: "Garmin PCX5 waypoint file",
    extensions: &["wpt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
