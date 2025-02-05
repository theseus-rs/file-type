use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858460: FileFormat = FileFormat {
    id: 105_858_460,
    source_type: SourceType::Wikidata,
    name: "EnergyPlus Weather data",
    extensions: &["epw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x4F, 0x43, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x2C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
