use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851786: FileFormat = FileFormat {
    id: 105_851_786,
    puid: "wikidata/105851786",
    name: "SETI@Home results",
    extensions: &["sah"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x72, 0x65, 0x73, 0x75, 0x6C, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
