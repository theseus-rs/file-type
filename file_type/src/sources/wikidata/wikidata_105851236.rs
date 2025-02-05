use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851236: FileFormat = FileFormat {
    id: 105_851_236,
    source_type: SourceType::Wikidata,
    name: "eMule Web Interface template",
    extensions: &["tmpl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
