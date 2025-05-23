use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856543: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_543,
        source_type: SourceType::Wikidata,
        name: "WinFellow Configuration",
        extensions: &["wfc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
