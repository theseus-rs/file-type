use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858603: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_603,
        source_type: SourceType::Wikidata,
        name: "C64 8x8 font bitmap",
        extensions: &["64c"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
