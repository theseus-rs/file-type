use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967186: FileFormat = FileFormat {
    id: 27_967_186,
    source_type: SourceType::Wikidata,
    name: "FunkTracker module",
    extensions: &["fnk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x75, 0x6E, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
