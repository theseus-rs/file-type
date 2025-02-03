use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854991: FileFormat = FileFormat {
    id: 105_854_991,
    source_type: SourceType::Wikidata,
    name: "ELI 5750 compressed archive",
    extensions: &["eli"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x72, 0x61, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
