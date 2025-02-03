use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854591: FileFormat = FileFormat {
    id: 105_854_591,
    source_type: SourceType::Wikidata,
    name: "Limit compressed archive",
    extensions: &["lim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
