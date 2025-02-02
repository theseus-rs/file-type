use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858603: FileFormat = FileFormat {
    id: 105_858_603,
    source_type: SourceType::Wikidata,
    name: "C64 8x8 font bitmap",
    extensions: &["64c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
