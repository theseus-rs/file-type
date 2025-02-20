use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_131,
        source_type: SourceType::Wikidata,
        name: "CPython 2.2 bytecode",
        extensions: &["pyc"],
        media_types: &["application/x-python-bytecode"],
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
