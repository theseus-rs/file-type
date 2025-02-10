use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858063: FileFormat = FileFormat {
    id: 105_858_063,
    source_type: SourceType::Wikidata,
    name: "ISI gMotor MAS type 0 game data archive",
    extensions: &["bmw", "gtl", "gtr", "mas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4D, 0x4F, 0x54, 0x4F, 0x52, 0x4D, 0x41, 0x53, 0x31, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
