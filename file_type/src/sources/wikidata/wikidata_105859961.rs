use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859961: FileFormat = FileFormat {
    id: 105_859_961,
    puid: "wikidata/105859961",
    name: "Vuze link",
    extensions: &["vuze"],
    media_types: &["application/x-bittorrent"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x34, 0x3A, 0x76, 0x75, 0x7A, 0x65, 0x64, 0x31, 0x30, 0x3A, 0x63, 0x6F,
                    0x6D, 0x70, 0x6F, 0x6E, 0x65, 0x6E, 0x74, 0x73, 0x6C, 0x64, 0x37, 0x3A, 0x63,
                    0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
