use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864157: FileFormat = FileFormat {
    id: 105_864_157,
    source_type: SourceType::Wikidata,
    name: "MO3 module",
    extensions: &["mo3"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
