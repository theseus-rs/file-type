use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863717: FileFormat = FileFormat {
    id: 105_863_717,
    source_type: SourceType::Wikidata,
    name: "HP ME10 database (ASCII)",
    extensions: &["mi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x7E, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
