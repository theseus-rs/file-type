use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854913: FileFormat = FileFormat {
    id: 105_854_913,
    puid: "wikidata/105854913",
    name: "STOS Sample",
    extensions: &["sam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4F, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
