use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851048: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_048,
        source_type: SourceType::Wikidata,
        name: "Cargo manifest",
        extensions: &["toml"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
