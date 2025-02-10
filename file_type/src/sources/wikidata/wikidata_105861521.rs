use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861521: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_521,
        source_type: SourceType::Wikidata,
        name: "Lua bytecode (generic)",
        extensions: &["out"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
