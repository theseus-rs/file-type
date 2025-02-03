use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851906: FileFormat = FileFormat {
    id: 105_851_906,
    source_type: SourceType::Wikidata,
    name: "Butcher Shape",
    extensions: &["shape"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x98, 0x76, 0xAB, 0xCD])],
            },
        }],
    }],
    related_formats: &[],
};
