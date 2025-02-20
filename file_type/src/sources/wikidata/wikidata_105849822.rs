use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849822: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_822,
        source_type: SourceType::Wikidata,
        name: "Painter 3D Contour",
        extensions: &["cnt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6E, 0x74, 0x0A, 0x41, 0x6C, 0x6C, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
