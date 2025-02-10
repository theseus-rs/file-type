use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_5381415: FileType = FileType {
    file_format: &FileFormat {
        id: 5_381_415,
        source_type: SourceType::Wikidata,
        name: "Envoy",
        extensions: &["evy"],
        media_types: &["application/x-envoy"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xB2, 0x97, 0xE1, 0x69])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
