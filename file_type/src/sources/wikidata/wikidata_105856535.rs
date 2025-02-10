use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856535: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_535,
        source_type: SourceType::Wikidata,
        name: "WarCraft III Recorded Game File",
        extensions: &["w3g"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x61, 0x72, 0x63, 0x72, 0x61, 0x66, 0x74, 0x20, 0x49, 0x49, 0x49,
                        0x20, 0x72, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x65, 0x64, 0x20, 0x67, 0x61,
                        0x6D, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
