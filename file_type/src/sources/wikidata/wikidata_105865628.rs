use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865628: FileFormat = FileFormat {
    id: 105_865_628,
    source_type: SourceType::Wikidata,
    name: "Pacific Warrior 2: Dogfight game data archive",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x4B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
