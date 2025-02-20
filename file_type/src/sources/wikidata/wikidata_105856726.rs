use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856726: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_726,
        source_type: SourceType::Wikidata,
        name: "KSDev ThemeEngine theme/skin",
        extensions: &["uskn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x55, 0x43, 0x54, 0x00, 0x11, 0x00, 0x00, 0x00, 0x54, 0x68, 0x65,
                        0x6D, 0x65, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x54, 0x68, 0x65,
                        0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
