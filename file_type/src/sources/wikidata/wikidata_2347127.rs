use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2347127: FileType = FileType {
    file_format: &FileFormat {
        id: 2_347_127,
        source_type: SourceType::Wikidata,
        name: "Compressed image format",
        extensions: &["ciso", "cso", "wbi"],
        media_types: &["application/octet-stream", "application/x-compressed-iso"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x49, 0x53, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
