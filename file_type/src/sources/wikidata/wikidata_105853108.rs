use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853108: FileFormat = FileFormat {
    id: 105_853_108,
    source_type: SourceType::Wikidata,
    name: "Winamp Signal Processing Studio DSP-Effect",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x50, 0x53, 0x20, 0x50, 0x52, 0x45, 0x53, 0x45, 0x54, 0x5D, 0x0D,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
