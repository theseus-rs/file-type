use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856894: FileFormat = FileFormat {
    id: 105_856_894,
    source_type: SourceType::Wikidata,
    name: "Guitar Pro v3 tablature",
    extensions: &["gp3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x46, 0x49, 0x43, 0x48, 0x49, 0x45, 0x52, 0x20, 0x47, 0x55, 0x49, 0x54,
                    0x41, 0x52, 0x20, 0x50, 0x52, 0x4F, 0x20, 0x76, 0x33, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
