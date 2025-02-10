use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861163: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_163,
        source_type: SourceType::Wikidata,
        name: "Amiga Hunk library/object code",
        extensions: &["lib", "obj"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0x00, 0x03, 0xE7])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x30, 0x00, 0x00, 0x00])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
