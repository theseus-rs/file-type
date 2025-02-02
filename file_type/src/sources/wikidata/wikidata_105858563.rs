use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858563: FileFormat = FileFormat {
    id: 105_858_563,
    source_type: SourceType::Wikidata,
    name: "Truevision TGA/TARGA bitmap (RLE encoded, RGB image)",
    extensions: &["tga"],
    media_types: &["image/x-tga"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
