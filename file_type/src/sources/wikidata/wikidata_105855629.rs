use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855629: FileFormat = FileFormat {
    id: 105_855_629,
    source_type: SourceType::Wikidata,
    name: "UVMapper object",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61,
                    0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x55, 0x56, 0x4D, 0x61, 0x70, 0x70,
                    0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
