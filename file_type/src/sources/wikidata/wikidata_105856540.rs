use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856540: FileFormat = FileFormat {
    id: 105_856_540,
    source_type: SourceType::Wikidata,
    name: "Altera Waveform Design File",
    extensions: &["wdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x44, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
