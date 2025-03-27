use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130416191: FileType = FileType {
    file_format: &FileFormat {
        id: 130_416_191,
        source_type: SourceType::Wikidata,
        name: "Fuchsia archive format",
        extensions: &["far"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xC8, 0xBF, 0x0B, 0x48, 0xAD, 0xAB, 0xC5, 0x11,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
