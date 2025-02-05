use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7323389: FileFormat = FileFormat {
    id: 7_323_389,
    source_type: SourceType::Wikidata,
    name: "Rich Music Format",
    extensions: &["rmf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x52, 0x45, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
