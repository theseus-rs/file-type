use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854169: FileFormat = FileFormat {
    id: 105_854_169,
    source_type: SourceType::Wikidata,
    name: "ARHANGEL compressed archive",
    extensions: &["lg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
