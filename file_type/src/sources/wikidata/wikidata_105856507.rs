use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856507: FileFormat = FileFormat {
    id: 105_856_507,
    source_type: SourceType::Wikidata,
    name: "Multibit Bitcoin wallet",
    extensions: &["wallet"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x16, 0x6F, 0x72, 0x67, 0x2E, 0x62, 0x69, 0x74, 0x63, 0x6F, 0x69, 0x6E,
                    0x2E, 0x70, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
