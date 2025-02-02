use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854547: FileFormat = FileFormat {
    id: 105_854_547,
    source_type: SourceType::Wikidata,
    name: "AirZip FileSECURE format (original only)",
    extensions: &["azs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD3, 0x41, 0x5A, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
