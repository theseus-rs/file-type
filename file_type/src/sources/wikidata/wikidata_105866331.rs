use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_331,
        source_type: SourceType::Wikidata,
        name: "PostgreSQL database dump",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x47, 0x44, 0x4D, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
