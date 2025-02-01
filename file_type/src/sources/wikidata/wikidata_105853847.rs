use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853847: FileFormat = FileFormat {
    id: 105_853_847,
    puid: "wikidata/105853847",
    name: "ERI compressed archive",
    extensions: &["eri"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6E, 0x69, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
