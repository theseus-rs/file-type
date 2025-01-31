use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855336: FileFormat = FileFormat {
    id: 105_855_336,
    puid: "wikidata/105855336",
    name: "Form Master Form (v4.0)",
    extensions: &["frm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0xB9, 0xB2])],
            },
        }],
    }],
    related_formats: &[],
};
