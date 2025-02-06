use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850235: FileFormat = FileFormat {
    id: 105_850_235,
    source_type: SourceType::Wikidata,
    name: "HTML Help Collection",
    extensions: &["col"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x58, 0x4D, 0x4C, 0x3E, 0x0D, 0x0A, 0x3C, 0x48, 0x54, 0x4D, 0x4C, 0x48,
                    0x65, 0x6C, 0x70, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                    0x3E, 0x0D, 0x0A, 0x3C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
