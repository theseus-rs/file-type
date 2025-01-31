use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851384: FileFormat = FileFormat {
    id: 105_851_384,
    puid: "wikidata/105851384",
    name: "Dynamic Publisher Text",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20, 0x50, 0x75, 0x62, 0x6C, 0x69,
                    0x73, 0x68, 0x65, 0x72, 0x20, 0x54, 0x65, 0x78, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
