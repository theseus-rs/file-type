use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850514: FileFormat = FileFormat {
    id: 105_850_514,
    source_type: SourceType::Wikidata,
    name: "Cinemaware music",
    extensions: &["cin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x4C, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
