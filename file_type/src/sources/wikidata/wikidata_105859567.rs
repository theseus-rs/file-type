use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859567: FileFormat = FileFormat {
    id: 105_859_567,
    puid: "wikidata/105859567",
    name: "Vice Flip List",
    extensions: &["vfl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x56, 0x69, 0x63, 0x65, 0x20, 0x66, 0x6C, 0x69, 0x70, 0x6C, 0x69,
                    0x73, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
