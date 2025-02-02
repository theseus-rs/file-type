use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863026: FileFormat = FileFormat {
    id: 105_863_026,
    source_type: SourceType::Wikidata,
    name: "DCAlice snapshot",
    extensions: &["mrx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x06, 0x33, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
