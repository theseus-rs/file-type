use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856247: FileFormat = FileFormat {
    id: 105_856_247,
    source_type: SourceType::Wikidata,
    name: "TomTom traffic data",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x41, 0x56, 0x54, 0x52, 0x41, 0x46, 0x46, 0x49, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
