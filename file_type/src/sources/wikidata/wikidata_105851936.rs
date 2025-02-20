use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_936,
        source_type: SourceType::Wikidata,
        name: "SQLite Zipvfs compressed database",
        extensions: &["sqlite"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x56, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
