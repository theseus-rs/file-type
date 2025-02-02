use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857652: FileFormat = FileFormat {
    id: 105_857_652,
    source_type: SourceType::Wikidata,
    name: "ZX Wafadrive Wafer image",
    extensions: &["wdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x57, 0x61, 0x66, 0x65, 0x72, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
