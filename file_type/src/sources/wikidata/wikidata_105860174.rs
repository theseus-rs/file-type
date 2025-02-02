use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860174: FileFormat = FileFormat {
    id: 105_860_174,
    source_type: SourceType::Wikidata,
    name: "Real C64 SID tune",
    extensions: &["sid"],
    media_types: &["audio/x-sid"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x49, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
