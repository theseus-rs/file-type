use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859582: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_582,
        source_type: SourceType::Wikidata,
        name: "Visual Studio wizard",
        extensions: &["vsz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x53, 0x57, 0x49, 0x5A, 0x41, 0x52, 0x44, 0x20, 0x37, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
