use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859188: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_188,
        source_type: SourceType::Wikidata,
        name: "Truevision TGA/TARGA bitmap (RLE encoded, RGB image, palette)",
        extensions: &["tga"],
        media_types: &["image/x-tga"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x0A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
