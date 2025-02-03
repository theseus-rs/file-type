use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858320: FileFormat = FileFormat {
    id: 105_858_320,
    source_type: SourceType::Wikidata,
    name: "SimCity 4 Exemplar (binary)",
    extensions: &["exmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x51, 0x5A, 0x42, 0x31, 0x23, 0x23, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
