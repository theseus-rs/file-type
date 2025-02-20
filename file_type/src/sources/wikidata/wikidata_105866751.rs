use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866751: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_751,
        source_type: SourceType::Wikidata,
        name: "PlayStation 3 Theme",
        extensions: &["p3t"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x33, 0x54, 0x46, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x40,
                        0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
