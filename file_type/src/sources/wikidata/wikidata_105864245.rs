use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864245: FileFormat = FileFormat {
    id: 105_864_245,
    puid: "wikidata/105864245",
    name: "Palm Markup Language",
    extensions: &["pml"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5C, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
