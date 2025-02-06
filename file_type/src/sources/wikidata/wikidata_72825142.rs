use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72825142: FileFormat = FileFormat {
    id: 72_825_142,
    source_type: SourceType::Wikidata,
    name: "Ortho 3D Model",
    extensions: &["O3M"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x33, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
