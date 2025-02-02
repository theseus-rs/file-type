use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854698: FileFormat = FileFormat {
    id: 105_854_698,
    source_type: SourceType::Wikidata,
    name: "Hamarsoft HAP compressed archive (v2.10)",
    extensions: &["hap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x91, 0x4A, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
