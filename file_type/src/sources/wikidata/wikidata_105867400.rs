use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867400: FileFormat = FileFormat {
    id: 105_867_400,
    source_type: SourceType::Wikidata,
    name: "Navitel 2.0 Map",
    extensions: &["ntm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4E, 0x54, 0x4D, 0x07, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
