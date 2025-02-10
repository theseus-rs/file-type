use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853266: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_266,
        source_type: SourceType::Wikidata,
        name: "Daemon Tools Pro disk image",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x54, 0x50, 0x52, 0x4F, 0x2D, 0x53, 0x61, 0x76, 0x65, 0x64, 0x20,
                        0x44, 0x69, 0x73, 0x6B, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
