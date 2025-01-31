use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28758112: FileFormat = FileFormat {
    id: 28_758_112,
    puid: "wikidata/28758112",
    name: "Internet Explorer history file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x20, 0x55, 0x72, 0x6C, 0x43, 0x61, 0x63,
                    0x68, 0x65, 0x20, 0x4D, 0x4D, 0x46, 0x20, 0x56, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
