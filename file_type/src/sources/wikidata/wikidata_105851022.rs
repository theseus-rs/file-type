use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851022: FileFormat = FileFormat {
    id: 105_851_022,
    source_type: SourceType::Wikidata,
    name: "Panasonic JR Cassette image",
    extensions: &["cjr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
