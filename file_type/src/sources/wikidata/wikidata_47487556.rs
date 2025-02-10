use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_47487556: FileType = FileType {
    file_format: &FileFormat {
        id: 47_487_556,
        source_type: SourceType::Wikidata,
        name: "TCR ebook",
        extensions: &["tcr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x21, 0x38, 0x2D, 0x42, 0x69, 0x74, 0x21, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
