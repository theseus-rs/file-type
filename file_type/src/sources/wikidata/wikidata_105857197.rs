use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857197: FileFormat = FileFormat {
    id: 105_857_197,
    source_type: SourceType::Wikidata,
    name: "Sim //e Virtual Hard Drive image",
    extensions: &["hdv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x4D, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x5F, 0x48, 0x44, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
