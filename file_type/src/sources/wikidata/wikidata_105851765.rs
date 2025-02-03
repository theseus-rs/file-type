use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851765: FileFormat = FileFormat {
    id: 105_851_765,
    source_type: SourceType::Wikidata,
    name: "Lotus Metro Screen Driver",
    extensions: &["sdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x04, 0x00, 0x44, 0x69, 0x73, 0x70, 0x6C, 0x61, 0x79, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
