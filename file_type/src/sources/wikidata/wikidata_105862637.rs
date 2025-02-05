use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862637: FileFormat = FileFormat {
    id: 105_862_637,
    source_type: SourceType::Wikidata,
    name: "Mediatek Font",
    extensions: &["mtf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4F, 0x4E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
