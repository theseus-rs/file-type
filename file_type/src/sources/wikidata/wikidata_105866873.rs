use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866873: FileFormat = FileFormat {
    id: 105_866_873,
    source_type: SourceType::Wikidata,
    name: "jEEPers Program Configuration file (without rem)",
    extensions: &["pfg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x49, 0x6E, 0x66, 0x6F, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
