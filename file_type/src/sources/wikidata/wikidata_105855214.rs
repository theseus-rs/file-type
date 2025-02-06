use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855214: FileFormat = FileFormat {
    id: 105_855_214,
    source_type: SourceType::Wikidata,
    name: "Reunion Family tree (v9)",
    extensions: &["familyfile"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x53, 0x44, 0x55, 0x39, 0x55, 0x7E, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
