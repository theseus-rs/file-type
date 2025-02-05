use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865594: FileFormat = FileFormat {
    id: 105_865_594,
    source_type: SourceType::Wikidata,
    name: "P64 NRZI flux pulse disk image",
    extensions: &["p64"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x36, 0x34, 0x2D, 0x31, 0x35, 0x34, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
