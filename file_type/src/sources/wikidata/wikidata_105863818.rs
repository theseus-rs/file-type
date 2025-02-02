use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863818: FileFormat = FileFormat {
    id: 105_863_818,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help (old)",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x65, 0x87])],
            },
        }],
    }],
    related_formats: &[],
};
