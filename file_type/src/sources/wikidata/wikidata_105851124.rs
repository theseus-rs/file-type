use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851124: FileFormat = FileFormat {
    id: 105_851_124,
    source_type: SourceType::Wikidata,
    name: "Telmac 600 program",
    extensions: &["tmc600"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x54, 0x4D, 0x43, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
