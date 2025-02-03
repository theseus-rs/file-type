use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853693: FileFormat = FileFormat {
    id: 105_853_693,
    source_type: SourceType::Wikidata,
    name: "AC3D model",
    extensions: &["ac3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x33, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
