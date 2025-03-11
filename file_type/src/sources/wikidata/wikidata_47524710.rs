use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47524710: FileType = FileType {
    file_format: &FileFormat {
        id: 47_524_710,
        source_type: SourceType::Wikidata,
        name: "Mach-O file format (32bit)",
        extensions: &[],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFE, 0xED, 0xFA, 0xCE])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xCE, 0xFA, 0xED, 0xFE])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
