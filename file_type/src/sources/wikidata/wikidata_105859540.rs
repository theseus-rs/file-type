use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859540: FileFormat = FileFormat {
    id: 105_859_540,
    source_type: SourceType::Wikidata,
    name: "Weston CAPture video (LE)",
    extensions: &["wcap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x43, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
