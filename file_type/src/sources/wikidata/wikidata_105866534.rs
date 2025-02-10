use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_534,
        source_type: SourceType::Wikidata,
        name: "Massive game data",
        extensions: &["pak"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x46, 0x49, 0x4C, 0x45, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
