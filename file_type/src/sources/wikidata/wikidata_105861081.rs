use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861081: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_081,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Protected LISP code",
        extensions: &["lsp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x50, 0x52, 0x4F, 0x54,
                        0x45, 0x43, 0x54, 0x45, 0x44, 0x20, 0x4C, 0x49, 0x53, 0x50, 0x20, 0x66,
                        0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
