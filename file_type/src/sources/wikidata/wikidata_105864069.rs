use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864069: FileFormat = FileFormat {
    id: 105_864_069,
    puid: "wikidata/105864069",
    name: "PlayStation RSD Material (gen)",
    extensions: &["mat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x4D, 0x41, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
