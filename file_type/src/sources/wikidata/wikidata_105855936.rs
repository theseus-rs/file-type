use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855936: FileFormat = FileFormat {
    id: 105_855_936,
    source_type: SourceType::Wikidata,
    name: "Isearch Database Info",
    extensions: &["dbi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2B, 0x44, 0x62, 0x49, 0x6E, 0x66, 0x6F, 0x0A, 0x20, 0x2B, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x4E, 0x75, 0x6D, 0x62, 0x65, 0x72, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
