use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4839791: FileType = FileType {
    file_format: &FileFormat {
        id: 4_839_791,
        source_type: SourceType::Wikidata,
        name: "Backup-file Format",
        extensions: &[],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x09, 0x00, 0x6F, 0xEA])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x09, 0x00, 0x6B, 0xEA])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
