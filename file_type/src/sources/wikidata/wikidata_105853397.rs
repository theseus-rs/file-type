use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853397: FileFormat = FileFormat {
    id: 105_853_397,
    source_type: SourceType::Wikidata,
    name: "Yamaha EX5 waveforms format",
    extensions: &["s1m"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x31, 0x32, 0x30, 0x30, 0x53, 0x50, 0x41, 0x20, 0x20, 0x56, 0x31,
                    0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
