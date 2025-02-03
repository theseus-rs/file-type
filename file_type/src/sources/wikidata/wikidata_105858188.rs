use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858188: FileFormat = FileFormat {
    id: 105_858_188,
    source_type: SourceType::Wikidata,
    name: "EEDraw Drawing",
    extensions: &["eed"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x45, 0x44, 0x72, 0x61, 0x77, 0x20, 0x45, 0x45, 0x44, 0x52, 0x41, 0x57,
                    0x20, 0x50, 0x52, 0x4F, 0x47, 0x52, 0x41, 0x4D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
