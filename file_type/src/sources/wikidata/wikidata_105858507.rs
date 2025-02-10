use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858507: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_507,
        source_type: SourceType::Wikidata,
        name: "Nintendo Binary File Sound ARchive",
        extensions: &["bfsar"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x53, 0x41, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
