use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_16959607: FileType = FileType {
    file_format: &FileFormat {
        id: 16_959_607,
        source_type: SourceType::Wikidata,
        name: "MBM",
        extensions: &["mbm"],
        media_types: &["application/octet-stream", "image/x-epoc-mbm"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x44, 0x41, 0x54, 0x41, 0x47, 0x72, 0x50, 0x3F,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x37, 0x00, 0x00, 0x10, 0x42, 0x00, 0x00, 0x10,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
