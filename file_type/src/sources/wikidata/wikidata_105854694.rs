use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854694: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_694,
        source_type: SourceType::Wikidata,
        name: "Advantage Data Server table",
        extensions: &["adt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x64, 0x76, 0x61, 0x6E, 0x74, 0x61, 0x67, 0x65, 0x20, 0x54, 0x61,
                        0x62, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
