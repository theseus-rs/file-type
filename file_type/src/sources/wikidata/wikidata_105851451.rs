use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851451: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_451,
        source_type: SourceType::Wikidata,
        name: "MegaPaint keyboard layout",
        extensions: &["ttb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x54, 0x54, 0x42, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
