use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854918: FileFormat = FileFormat {
    id: 105_854_918,
    puid: "wikidata/105854918",
    name: "Rigaku Smartlab Pole Figure raw format",
    extensions: &["asc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x54, 0x59, 0x50, 0x45, 0x09, 0x09, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
