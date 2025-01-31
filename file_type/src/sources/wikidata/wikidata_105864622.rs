use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864622: FileFormat = FileFormat {
    id: 105_864_622,
    puid: "wikidata/105864622",
    name: "Protege classes",
    extensions: &["pont"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
