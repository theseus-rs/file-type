use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857082: FileFormat = FileFormat {
    id: 105_857_082,
    source_type: SourceType::Wikidata,
    name: "GPS Tuner map slices calibration data",
    extensions: &["gsi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x70, 0x20, 0x53, 0x6C, 0x69, 0x63, 0x65, 0x73, 0x20, 0x43, 0x61,
                    0x6C, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x64, 0x61, 0x74,
                    0x61, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
