use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76515294: FileFormat = FileFormat {
    id: 76_515_294,
    source_type: SourceType::Wikidata,
    name: "WannaCry ransomware encrypted",
    extensions: &["wncry"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x41, 0x4E, 0x41, 0x43, 0x52, 0x59, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
