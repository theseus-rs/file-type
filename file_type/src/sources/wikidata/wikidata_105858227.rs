use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858227: FileFormat = FileFormat {
    id: 105_858_227,
    source_type: SourceType::Wikidata,
    name: "EncryptOnClick encrypted",
    extensions: &["eoc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x01, 0x00, 0x63, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
