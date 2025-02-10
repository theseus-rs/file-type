use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865313: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_313,
        source_type: SourceType::Wikidata,
        name: "Solid Edge Packaged Collaboration File",
        extensions: &["pcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
