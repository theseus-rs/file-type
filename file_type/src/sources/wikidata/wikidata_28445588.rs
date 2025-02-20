use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445588: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_588,
        source_type: SourceType::Wikidata,
        name: "ALZ",
        extensions: &["alz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4C, 0x5A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
