use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855720: FileFormat = FileFormat {
    id: 105_855_720,
    source_type: SourceType::Wikidata,
    name: "Organya 2 module",
    extensions: &["org"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x72, 0x67, 0x2D, 0x30, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
