use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850215: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_215,
        source_type: SourceType::Wikidata,
        name: "CD Label Pro project",
        extensions: &["cdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xBE, 0xBA, 0x0C, 0x43, 0x44, 0x2D, 0x4C, 0x41, 0x42, 0x45, 0x4C, 0x20,
                        0x50, 0x52, 0x4F, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
