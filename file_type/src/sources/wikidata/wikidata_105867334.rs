use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867334: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_334,
        source_type: SourceType::Wikidata,
        name: "NonoPocket nonogram",
        extensions: &["ngb"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x4F, 0x4E, 0x4F, 0x47, 0x52, 0x41, 0x4D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
