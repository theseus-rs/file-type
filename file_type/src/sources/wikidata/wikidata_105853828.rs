use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853828: FileFormat = FileFormat {
    id: 105_853_828,
    source_type: SourceType::Wikidata,
    name: "Atari800Win Plus Trainer",
    extensions: &["a8t"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x38, 0x54, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
