use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850866: FileFormat = FileFormat {
    id: 105_850_866,
    source_type: SourceType::Wikidata,
    name: "2D Fighter Maker 2nd stage data",
    extensions: &["stage"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x44, 0x4B, 0x47, 0x54, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
