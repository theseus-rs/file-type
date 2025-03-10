use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762928: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_928,
        source_type: SourceType::Wikidata,
        name: "Xbox Dashboard data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x49, 0x50, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
