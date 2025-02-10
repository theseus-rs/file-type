use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866145: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_145,
        source_type: SourceType::Wikidata,
        name: "PxTone Collage module (protected)",
        extensions: &["pttune"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x54, 0x54, 0x55, 0x4E, 0x45, 0x2D, 0x2D, 0x32, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
