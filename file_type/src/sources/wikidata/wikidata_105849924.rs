use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849924: FileFormat = FileFormat {
    id: 105_849_924,
    source_type: SourceType::Wikidata,
    name: "Codebreaker save",
    extensions: &["cbs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x46, 0x55, 0x00, 0x40, 0x1F, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
