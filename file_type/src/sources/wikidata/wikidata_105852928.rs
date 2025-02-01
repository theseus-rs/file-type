use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852928: FileFormat = FileFormat {
    id: 105_852_928,
    puid: "wikidata/105852928",
    name: "Splint compressed data",
    extensions: &["spl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x93, 0xB9, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
