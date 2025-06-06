use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55429372: FileType = FileType {
    file_format: &FileFormat {
        id: 55_429_372,
        source_type: SourceType::Wikidata,
        name: "Python bytecode, version 3.5",
        extensions: &["pyc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x0D, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
