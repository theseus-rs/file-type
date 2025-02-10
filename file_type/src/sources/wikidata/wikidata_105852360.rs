use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852360: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_360,
        source_type: SourceType::Wikidata,
        name: "Shot Online Resource",
        extensions: &["sor"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x68, 0x6F, 0x74, 0x4F, 0x6E, 0x6C, 0x69, 0x6E, 0x65, 0x20, 0x52,
                        0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x56, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
