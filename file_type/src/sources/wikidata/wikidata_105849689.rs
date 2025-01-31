use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849689: FileFormat = FileFormat {
    id: 105_849_689,
    puid: "wikidata/105849689",
    name: "PlayStation RSD Coordinates (gen)",
    extensions: &["cod"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x43, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
