use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856159: FileFormat = FileFormat {
    id: 105_856_159,
    puid: "wikidata/105856159",
    name: "Dragon UnPACKer Look",
    extensions: &["dulk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x55, 0x4C, 0x4B, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
