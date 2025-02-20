use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866221: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_221,
        source_type: SourceType::Wikidata,
        name: "CPython 3.0 bytecode",
        extensions: &["pyc"],
        media_types: &["application/x-python-bytecode"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x0C, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
