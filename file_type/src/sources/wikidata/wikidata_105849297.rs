use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849297: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_297,
        source_type: SourceType::Wikidata,
        name: "Nintendo Yaz0 compressed data",
        extensions: &["arc", "szs", "yaz0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x61, 0x7A, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
