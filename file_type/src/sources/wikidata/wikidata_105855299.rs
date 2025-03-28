use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855299: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_299,
        source_type: SourceType::Wikidata,
        name: "FFR archiv format",
        extensions: &["ffr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x46, 0x52, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x20, 0x66,
                        0x6F, 0x72, 0x6D, 0x61, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
