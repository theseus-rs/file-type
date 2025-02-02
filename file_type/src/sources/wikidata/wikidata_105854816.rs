use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854816: FileFormat = FileFormat {
    id: 105_854_816,
    source_type: SourceType::Wikidata,
    name: "Pretty Simple Archiver compressed archive",
    extensions: &["psa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x41, 0x01, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
