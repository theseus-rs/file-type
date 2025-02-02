use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855636: FileFormat = FileFormat {
    id: 105_855_636,
    source_type: SourceType::Wikidata,
    name: "Visi On Overlay",
    extensions: &["ovs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x00, 0x07, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
