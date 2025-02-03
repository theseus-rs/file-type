use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95985268: FileFormat = FileFormat {
    id: 95_985_268,
    source_type: SourceType::Wikidata,
    name: "Affymetrix CEL file format",
    extensions: &["cel"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x43, 0x45, 0x4C, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
