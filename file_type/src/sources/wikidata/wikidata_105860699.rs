use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860699: FileFormat = FileFormat {
    id: 105_860_699,
    puid: "wikidata/105860699",
    name: "Haven Resource data",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x61, 0x76, 0x65, 0x6E, 0x20, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63,
                    0x65, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
