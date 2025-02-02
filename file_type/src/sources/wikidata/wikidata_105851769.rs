use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851769: FileFormat = FileFormat {
    id: 105_851_769,
    source_type: SourceType::Wikidata,
    name: "Adventure SOS compiled walkthrough",
    extensions: &["sos"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x41, 0x64, 0x53, 0x4F, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
