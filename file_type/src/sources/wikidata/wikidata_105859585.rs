use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859585: FileFormat = FileFormat {
    id: 105_859_585,
    source_type: SourceType::Wikidata,
    name: "PlayStation single waveform data format",
    extensions: &["vag"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x41, 0x47, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
