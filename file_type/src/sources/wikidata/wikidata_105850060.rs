use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850060: FileFormat = FileFormat {
    id: 105_850_060,
    source_type: SourceType::Wikidata,
    name: "BoomTracker 4.0 instrument",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x55, 0x44, 0x2D, 0x46, 0x4D, 0x2D, 0x49, 0x6E, 0x73, 0x74, 0x72,
                    0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
