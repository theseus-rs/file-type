use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853949: FileFormat = FileFormat {
    id: 105_853_949,
    source_type: SourceType::Wikidata,
    name: "Dzip compressed archive (v2.x)",
    extensions: &["dz"],
    media_types: &["application/x-dzip"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x5A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
