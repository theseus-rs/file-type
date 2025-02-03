use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856710: FileFormat = FileFormat {
    id: 105_856_710,
    source_type: SourceType::Wikidata,
    name: "Unique Development Song/module",
    extensions: &["uds"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x8F, 0x4E, 0x47, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
