use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857482: FileFormat = FileFormat {
    id: 105_857_482,
    source_type: SourceType::Wikidata,
    name: "SMS 2D Mesh",
    extensions: &["2dm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x45, 0x53, 0x48, 0x32, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
