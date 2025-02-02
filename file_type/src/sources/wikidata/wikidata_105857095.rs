use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857095: FileFormat = FileFormat {
    id: 105_857_095,
    source_type: SourceType::Wikidata,
    name: "Gens Plus! movie capture",
    extensions: &["gir"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4D, 0x56, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
