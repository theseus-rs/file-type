use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853946: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_946,
        source_type: SourceType::Wikidata,
        name: "BZA compressed archive",
        extensions: &["bza"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x42, 0x5A, 0x32, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
