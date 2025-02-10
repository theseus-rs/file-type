use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853727: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_727,
        source_type: SourceType::Wikidata,
        name: "CD Autorun Creator package",
        extensions: &["arn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x44, 0x20, 0x41, 0x55, 0x54, 0x4F, 0x52, 0x55, 0x4E, 0x20, 0x43,
                        0x52, 0x45, 0x41, 0x54, 0x4F, 0x52, 0x20, 0x50, 0x41, 0x43, 0x4B, 0x41,
                        0x47, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
