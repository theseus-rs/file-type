use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_784695: FileType = FileType {
    file_format: &FileFormat {
        id: 784_695,
        source_type: SourceType::Wikidata,
        name: "ZIM",
        extensions: &["zim"],
        media_types: &["application/octet-stream", "text/plain"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x5A, 0x49, 0x4D, 0x04])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x5A, 0x49, 0x4D])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
