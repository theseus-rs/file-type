use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855061: FileFormat = FileFormat {
    id: 105_855_061,
    source_type: SourceType::Wikidata,
    name: "SBC compressed archive",
    extensions: &["sbc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x43, 0x1E])],
            },
        }],
    }],
    related_formats: &[],
};
