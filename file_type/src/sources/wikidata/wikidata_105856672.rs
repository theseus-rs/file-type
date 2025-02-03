use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856672: FileFormat = FileFormat {
    id: 105_856_672,
    source_type: SourceType::Wikidata,
    name: "USB Flashing Format",
    extensions: &["uf2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x46, 0x32, 0x0A, 0x57, 0x51, 0x5D, 0x9E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
