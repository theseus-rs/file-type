use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967540: FileFormat = FileFormat {
    id: 27_967_540,
    puid: "wikidata/27967540",
    name: "Cyber Paint Sequence",
    extensions: &["seq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xDB, 0x00, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
