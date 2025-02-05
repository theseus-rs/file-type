use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865116: FileFormat = FileFormat {
    id: 105_865_116,
    source_type: SourceType::Wikidata,
    name: "Pixilang compiled byte-code",
    extensions: &["pixicode"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x49, 0x58, 0x49, 0x43, 0x4F, 0x44, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
