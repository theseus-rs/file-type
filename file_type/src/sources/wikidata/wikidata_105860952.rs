use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860952: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_952,
        source_type: SourceType::Wikidata,
        name: "Lexmark Enhanced MetaFile",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x45, 0x4D, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
