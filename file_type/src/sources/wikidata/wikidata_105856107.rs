use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856107: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_107,
        source_type: SourceType::Wikidata,
        name: "Dis bytecode (signed)",
        extensions: &["dis"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x0E, 0x17, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
