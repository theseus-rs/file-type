use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1887604: FileType = FileType {
    file_format: &FileFormat {
        id: 1_887_604,
        source_type: SourceType::Wikidata,
        name: "X PixMap",
        extensions: &["xpm"],
        media_types: &["image/x-xpixmap", "image/xpm", "text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x20, 0x58, 0x50, 0x4D, 0x20, 0x2A, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
