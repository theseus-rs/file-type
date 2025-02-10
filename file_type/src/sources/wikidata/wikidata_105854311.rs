use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854311: FileFormat = FileFormat {
    id: 105_854_311,
    source_type: SourceType::Wikidata,
    name: "Scifer Archiver compressed Binary Archive",
    extensions: &["ba", "sen"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0xEA, 0xFA, 0xCE])],
            },
        }],
    }],
    related_formats: &[],
};
