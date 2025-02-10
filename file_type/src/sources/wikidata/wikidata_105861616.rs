use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_616,
        source_type: SourceType::Wikidata,
        name: "Take Command: 2nd Manassas game data archive",
        extensions: &["dat", "lsl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x01, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
