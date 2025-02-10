use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861793: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_793,
        source_type: SourceType::Wikidata,
        name: "Maple Common Binary (Amiga)",
        extensions: &["m"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x32, 0x96, 0x41, 0x00, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
