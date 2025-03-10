use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762785: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_785,
        source_type: SourceType::Wikidata,
        name: "XACT Wave Bank",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x42, 0x4E, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
