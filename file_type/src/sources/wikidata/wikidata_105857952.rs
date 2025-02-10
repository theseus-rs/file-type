use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857952: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_952,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto III model IDs and properties",
        extensions: &["ide"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x0D, 0x0A, 0x23, 0x20, 0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x49, 0x44,
                        0x73, 0x20, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
