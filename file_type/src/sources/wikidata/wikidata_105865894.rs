use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865894: FileFormat = FileFormat {
    id: 105_865_894,
    source_type: SourceType::Wikidata,
    name: "Spellbound game data archive",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x42, 0x50, 0x41, 0x4B, 0x20, 0x56, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
