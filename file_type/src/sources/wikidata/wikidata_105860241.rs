use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860241: FileFormat = FileFormat {
    id: 105_860_241,
    puid: "wikidata/105860241",
    name: "SCream Engine resource data",
    extensions: &["res"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x43, 0x72, 0x65, 0x61, 0x6D, 0x20, 0x72, 0x65, 0x73, 0x6F, 0x75, 0x72,
                    0x63, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
