use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861645: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_645,
        source_type: SourceType::Wikidata,
        name: "Lego Digital Designer data",
        extensions: &["lif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
