use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860529: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_529,
        source_type: SourceType::Wikidata,
        name: "Relocatable object file format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x44, 0x4F, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
