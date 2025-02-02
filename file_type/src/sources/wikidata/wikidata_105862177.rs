use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862177: FileFormat = FileFormat {
    id: 105_862_177,
    source_type: SourceType::Wikidata,
    name: "Fractal Forge 2.x fractal parameters",
    extensions: &["mnd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x11, 0x46, 0x72, 0x61, 0x63, 0x74, 0x61, 0x6C, 0x20, 0x46, 0x6F, 0x72, 0x67,
                    0x65, 0x20, 0x32, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
