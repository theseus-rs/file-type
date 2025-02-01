use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861653: FileFormat = FileFormat {
    id: 105_861_653,
    puid: "wikidata/105861653",
    name: "CP/M-86 library",
    extensions: &["l86"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA4, 0x07, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
