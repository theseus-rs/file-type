use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849951: FileFormat = FileFormat {
    id: 105_849_951,
    source_type: SourceType::Wikidata,
    name: "Doom Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x6F, 0x75, 0x73, 0x65, 0x5F, 0x73, 0x65, 0x6E, 0x73, 0x69, 0x74, 0x69,
                    0x76, 0x69, 0x74, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
