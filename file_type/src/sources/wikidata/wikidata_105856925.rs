use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856925: FileFormat = FileFormat {
    id: 105_856_925,
    puid: "wikidata/105856925",
    name: "GVA/GVA2000 Author lecture",
    extensions: &["gdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x42, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
