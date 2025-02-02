use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854282: FileFormat = FileFormat {
    id: 105_854_282,
    source_type: SourceType::Wikidata,
    name: "Authorware Shocked File (Map)",
    extensions: &["aam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x65, 0x72, 0x09, 0x30, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
