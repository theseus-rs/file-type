use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856282: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_282,
        source_type: SourceType::Wikidata,
        name: "Smart Software Document (v2.1)",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x80, 0x56, 0x02, 0x0A, 0x00, 0x0B, 0x4F, 0x0F, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
