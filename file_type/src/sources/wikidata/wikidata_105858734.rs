use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858734: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_734,
        source_type: SourceType::Wikidata,
        name: "Boom Box project (v1)",
        extensions: &["box"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x58, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
