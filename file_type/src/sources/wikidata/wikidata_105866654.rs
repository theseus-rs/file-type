use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866654: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_654,
        source_type: SourceType::Wikidata,
        name: "Sony PSP Theme file",
        extensions: &["ptf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x50, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
