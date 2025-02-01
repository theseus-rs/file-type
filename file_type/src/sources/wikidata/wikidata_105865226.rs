use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865226: FileFormat = FileFormat {
    id: 105_865_226,
    puid: "wikidata/105865226",
    name: "PolyWorks Polygonal format (v2.0)",
    extensions: &["pol"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4F, 0x4C, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x76, 0x32,
                    0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
