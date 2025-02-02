use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856085: FileFormat = FileFormat {
    id: 105_856_085,
    source_type: SourceType::Wikidata,
    name: "DMesh 3d model",
    extensions: &["dmz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
