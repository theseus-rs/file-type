use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855693: FileFormat = FileFormat {
    id: 105_855_693,
    source_type: SourceType::Wikidata,
    name: "CADRazor 3d model",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
