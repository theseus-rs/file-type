use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_154,
        source_type: SourceType::Wikidata,
        name: "86Box Floppy disk image",
        extensions: &["86f"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0x36, 0x42, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
