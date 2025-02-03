use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967192: FileFormat = FileFormat {
    id: 27_967_192,
    source_type: SourceType::Wikidata,
    name: "Graoumf Tracker module",
    extensions: &["gtk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x54, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
