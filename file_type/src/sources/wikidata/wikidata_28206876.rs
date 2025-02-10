use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206876: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_876,
        source_type: SourceType::Wikidata,
        name: "PCPaint PIC",
        extensions: &["clp", "pic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x00, 0xB8, 0x00, 0x00, 0x00, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
