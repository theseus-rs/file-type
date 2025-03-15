use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205595: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_595,
        source_type: SourceType::Wikidata,
        name: "Photoshop Thumbnail Cache",
        extensions: &["bc", "bct", "md0", "tb0"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x6C, 0x6E, 0x62, 0x74])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x68, 0x63, 0x61, 0x63])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x61, 0x74, 0x65, 0x6D])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
