use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857000: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_000,
        source_type: SourceType::Wikidata,
        name: "Generic Printer Description - Unidrv minidriver",
        extensions: &["gpd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
