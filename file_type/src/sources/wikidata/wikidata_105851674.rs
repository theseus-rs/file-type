use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851674: FileFormat = FileFormat {
    id: 105_851_674,
    source_type: SourceType::Wikidata,
    name: "EPOC Installation package (rel. 6)",
    extensions: &["sis"],
    media_types: &["x-epoc/x-sisx-app"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x12, 0x3A, 0x00, 0x10, 0x19, 0x04, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
