use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852298: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_298,
        source_type: SourceType::Wikidata,
        name: "SCD Square Enix Sound Container File",
        extensions: &["scd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x44, 0x42, 0x53, 0x53, 0x43, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
