use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855696: FileFormat = FileFormat {
    id: 105_855_696,
    source_type: SourceType::Wikidata,
    name: "Wavefront Object (created by Hexagon)",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x57, 0x61, 0x76, 0x65, 0x66, 0x72, 0x6F, 0x6E, 0x74, 0x20, 0x4F, 0x42,
                    0x4A, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
                    0x64, 0x20, 0x62, 0x79, 0x20, 0x48, 0x65, 0x78, 0x61, 0x67, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
