use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_784,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database Deferred Update Directory",
        extensions: &["sdo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x44, 0x4F, 0x20, 0x30, 0x30, 0x34, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
