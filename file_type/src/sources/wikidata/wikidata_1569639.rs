use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1569639: FileFormat = FileFormat {
    id: 1_569_639,
    puid: "wikidata/1569639",
    name: "Interchange File Format",
    extensions: &["iff"],
    media_types: &["application/x-iff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4F, 0x52, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
