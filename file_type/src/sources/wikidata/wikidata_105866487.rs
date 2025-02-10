use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866487: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_487,
        source_type: SourceType::Wikidata,
        name: "CPython 1.x bytecode",
        extensions: &["pyc"],
        media_types: &["application/x-python-bytecode"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x99, 0x4E, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
