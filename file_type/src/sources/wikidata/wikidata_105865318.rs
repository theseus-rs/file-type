use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865318: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_318,
        source_type: SourceType::Wikidata,
        name: "HP LaserJet Printer Cartridge Metric",
        extensions: &["pcm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0x0C, 0x10, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
