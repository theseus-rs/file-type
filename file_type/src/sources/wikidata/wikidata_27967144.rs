use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967144: FileFormat = FileFormat {
    id: 27_967_144,
    source_type: SourceType::Wikidata,
    name: "DisorderTracker 2 module",
    extensions: &["plm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4C, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
