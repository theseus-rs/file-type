use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858672: FileFormat = FileFormat {
    id: 105_858_672,
    source_type: SourceType::Wikidata,
    name: "ICE Book Reader eBook",
    extensions: &["book"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x43, 0x45, 0x20, 0x42, 0x6F, 0x6F, 0x6B, 0x20, 0x52, 0x65, 0x61, 0x64,
                    0x65, 0x72, 0x00, 0x28, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
