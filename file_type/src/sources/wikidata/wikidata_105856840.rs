use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856840: FileFormat = FileFormat {
    id: 105_856_840,
    source_type: SourceType::Wikidata,
    name: "Genital Save state (v1.2+)",
    extensions: &["gns"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4E, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
