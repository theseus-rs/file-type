use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858634: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_634,
        source_type: SourceType::Wikidata,
        name: "DB2 Bind (old)",
        extensions: &["bnd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFB, 0x49, 0x4E, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
