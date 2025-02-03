use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853305: FileFormat = FileFormat {
    id: 105_853_305,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics presentation (v2.x)",
    extensions: &["shw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
