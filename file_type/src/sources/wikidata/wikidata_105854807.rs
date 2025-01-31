use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854807: FileFormat = FileFormat {
    id: 105_854_807,
    puid: "wikidata/105854807",
    name: "Bit Archiver compressed archive",
    extensions: &["bit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x49, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
