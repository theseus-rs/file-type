use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858108: FileFormat = FileFormat {
    id: 105_858_108,
    source_type: SourceType::Wikidata,
    name: "CD-I disk image",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x02, 0x00, 0x02, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
