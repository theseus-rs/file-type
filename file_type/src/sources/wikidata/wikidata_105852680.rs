use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852680: FileFormat = FileFormat {
    id: 105_852_680,
    source_type: SourceType::Wikidata,
    name: "Respawn Entertainment game data archive",
    extensions: &["starpak"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x52, 0x50, 0x6B, 0x01, 0x00, 0x00, 0x00, 0xCB, 0xCB, 0xCB, 0xCB, 0xCB,
                    0xCB, 0xCB, 0xCB,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
