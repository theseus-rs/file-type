use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865486: FileFormat = FileFormat {
    id: 105_865_486,
    source_type: SourceType::Wikidata,
    name: "Adobe Photoshop Pattern",
    extensions: &["pat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x38, 0x42, 0x50, 0x54, 0x00, 0x01, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
