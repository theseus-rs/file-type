use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_228,
        source_type: SourceType::Wikidata,
        name: "CSC compressed archive",
        extensions: &["csc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x73, 0x79, 0x08])],
                },
            }],
        }],
        related_formats: &[],
    },
};
