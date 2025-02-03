use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857108: FileFormat = FileFormat {
    id: 105_857_108,
    source_type: SourceType::Wikidata,
    name: "Graphical Analysis 3 document",
    extensions: &["ga3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
