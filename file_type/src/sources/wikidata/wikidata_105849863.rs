use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849863: FileFormat = FileFormat {
    id: 105_849_863,
    source_type: SourceType::Wikidata,
    name: "CFast Animation",
    extensions: &["cft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x55, 0x43, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
