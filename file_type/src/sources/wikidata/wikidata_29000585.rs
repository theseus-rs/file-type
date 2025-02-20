use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000585: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_585,
        source_type: SourceType::Wikidata,
        name: "Dalvik VM DEX",
        extensions: &["dex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x65, 0x78, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
