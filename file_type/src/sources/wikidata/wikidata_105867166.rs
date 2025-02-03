use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867166: FileFormat = FileFormat {
    id: 105_867_166,
    source_type: SourceType::Wikidata,
    name: "NetStumbler NS1 log",
    extensions: &["ns1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x65, 0x74, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
