use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851331: FileFormat = FileFormat {
    id: 105_851_331,
    puid: "wikidata/105851331",
    name: "TinyDisk Meta File",
    extensions: &["tdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x54, 0x69, 0x6E, 0x79, 0x44, 0x69, 0x73, 0x6B, 0x46, 0x69, 0x6C,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
