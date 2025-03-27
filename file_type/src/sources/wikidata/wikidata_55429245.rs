use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55429245: FileType = FileType {
    file_format: &FileFormat {
        id: 55_429_245,
        source_type: SourceType::Wikidata,
        name: "Python bytecode, version 2.2",
        extensions: &["pyc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0xED, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
