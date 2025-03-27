use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55429271: FileType = FileType {
    file_format: &FileFormat {
        id: 55_429_271,
        source_type: SourceType::Wikidata,
        name: "Python bytecode, version 2.4",
        extensions: &["pyc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0xF2, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
