use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858077: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_077,
        source_type: SourceType::Wikidata,
        name: "IsoDraw Document",
        extensions: &["iso"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x53, 0x4F, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
