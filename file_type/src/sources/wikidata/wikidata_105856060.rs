use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856060: FileFormat = FileFormat {
    id: 105_856_060,
    puid: "wikidata/105856060",
    name: "Descent 2 alternative texture set",
    extensions: &["pog"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x4F, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
