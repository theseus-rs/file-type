use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_3400889: FileType = FileType {
    file_format: &FileFormat {
        id: 3_400_889,
        source_type: SourceType::Wikidata,
        name: "PowerPoint show",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xA0, 0x46, 0x1D, 0xF0])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x0F, 0x00, 0xE8, 0x03])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0x6E, 0x1E, 0xF0])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
