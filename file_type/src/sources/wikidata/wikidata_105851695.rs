use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851695: FileFormat = FileFormat {
    id: 105_851_695,
    source_type: SourceType::Wikidata,
    name: "Windows Live Messenger Log file",
    extensions: &["sqm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x51, 0x4D, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
