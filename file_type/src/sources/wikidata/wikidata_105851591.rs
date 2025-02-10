use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851591: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_591,
        source_type: SourceType::Wikidata,
        name: "Turbo Cross design",
        extensions: &["tbx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x9D, 0x65, 0x5D, 0x3E, 0xAB, 0x0E, 0x28, 0xD3, 0xE7, 0xBC, 0x8A, 0x34,
                        0x0D, 0xA2, 0xE9, 0x7D, 0x00, 0x05, 0x20, 0x1D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
