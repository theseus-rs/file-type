use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_786,
        source_type: SourceType::Wikidata,
        name: "XESS worksheet (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x3A, 0x3A, 0x58, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
