use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904468: FileFormat = FileFormat {
    id: 29_904_468,
    source_type: SourceType::Wikidata,
    name: "Quadruple D Archiver",
    extensions: &["qda"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x51, 0x44, 0x41, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
