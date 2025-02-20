use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859295: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_295,
        source_type: SourceType::Wikidata,
        name: "Playstation 3 icon",
        extensions: &["gim"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x47, 0x2E, 0x30, 0x30, 0x2E, 0x31, 0x50, 0x53, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
