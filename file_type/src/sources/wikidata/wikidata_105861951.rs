use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861951: FileFormat = FileFormat {
    id: 105_861_951,
    source_type: SourceType::Wikidata,
    name: "MATLAB Mac 64bit compiled function",
    extensions: &["mexmaci64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCF, 0xFA, 0xED, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
