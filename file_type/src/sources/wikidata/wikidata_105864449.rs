use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864449: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_449,
        source_type: SourceType::Wikidata,
        name: "CPython 2.5 bytecode",
        extensions: &["pyc"],
        media_types: &["application/x-python-bytecode"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB3, 0xF2, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
