use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853898: FileFormat = FileFormat {
    id: 105_853_898,
    source_type: SourceType::Wikidata,
    name: "Ashampoo Burning Studio project",
    extensions: &["ashprj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x73, 0x68, 0x70, 0x72, 0x6A, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
