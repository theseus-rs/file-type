use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851600: FileFormat = FileFormat {
    id: 105_851_600,
    source_type: SourceType::Wikidata,
    name: "MDL Transportable Graphics Format",
    extensions: &["tgf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
