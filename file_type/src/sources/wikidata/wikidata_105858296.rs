use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858296: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_296,
        source_type: SourceType::Wikidata,
        name: "Jasspa's MicroEmacs Macro File",
        extensions: &["emf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x20, 0x2D, 0x21, 0x2D, 0x20, 0x65, 0x6D, 0x66, 0x20, 0x2D, 0x21,
                        0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
