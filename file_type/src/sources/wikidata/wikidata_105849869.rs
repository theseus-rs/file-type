use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849869: FileFormat = FileFormat {
    id: 105_849_869,
    source_type: SourceType::Wikidata,
    name: "ODI link driver configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x49, 0x4E, 0x4B, 0x20, 0x44, 0x52, 0x49, 0x56, 0x45, 0x52, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
