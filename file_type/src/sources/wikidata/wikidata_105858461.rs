use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858461: FileFormat = FileFormat {
    id: 105_858_461,
    source_type: SourceType::Wikidata,
    name: "E-mu Emaxsynth sample",
    extensions: &["ez2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4D, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
