use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851070: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_070,
        source_type: SourceType::Wikidata,
        name: "TextEngine document (v4.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x66, 0x33, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
