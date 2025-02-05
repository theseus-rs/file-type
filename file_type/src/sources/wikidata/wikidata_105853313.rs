use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853313: FileFormat = FileFormat {
    id: 105_853_313,
    source_type: SourceType::Wikidata,
    name: "SAP Itutor tutorial",
    extensions: &["sim"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x69, 0x54, 0x75, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
