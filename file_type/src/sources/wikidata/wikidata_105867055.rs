use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867055: FileFormat = FileFormat {
    id: 105_867_055,
    source_type: SourceType::Wikidata,
    name: "SeeYou Waypoint",
    extensions: &["ndb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x20, 0x49, 0x4C, 0x45, 0x43, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
