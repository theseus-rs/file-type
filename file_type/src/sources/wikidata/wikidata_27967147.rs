use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967147: FileFormat = FileFormat {
    id: 27_967_147,
    source_type: SourceType::Wikidata,
    name: "Extended MOD",
    extensions: &["emd"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4D, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
