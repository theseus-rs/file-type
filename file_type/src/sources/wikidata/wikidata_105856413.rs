use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856413: FileFormat = FileFormat {
    id: 105_856_413,
    source_type: SourceType::Wikidata,
    name: "GFA Raytrace project data (hi-res)",
    extensions: &["wfh"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x66, 0x68, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
