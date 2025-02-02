use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849672: FileFormat = FileFormat {
    id: 105_849_672,
    source_type: SourceType::Wikidata,
    name: "WinUAE configuration Cache",
    extensions: &["cache"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x57, 0x69, 0x6E, 0x55, 0x41, 0x45, 0x20, 0x43, 0x6F, 0x6E,
                    0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2E, 0x43, 0x61,
                    0x63, 0x68, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
