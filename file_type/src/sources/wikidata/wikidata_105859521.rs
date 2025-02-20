use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859521: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_521,
        source_type: SourceType::Wikidata,
        name: "Ragnarok Online 2 game data archive",
        extensions: &["vdk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x44, 0x49, 0x53, 0x4B, 0x31, 0x2E, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
