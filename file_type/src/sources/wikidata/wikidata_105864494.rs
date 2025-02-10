use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864494: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_494,
        source_type: SourceType::Wikidata,
        name: "PageSetter II page (v1.x)",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1D, 0x19, 0x18, 0x17, 0x50, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74,
                        0x65, 0x72, 0x20, 0x49, 0x49, 0x20, 0x28, 0x56, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
