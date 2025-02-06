use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853729: FileFormat = FileFormat {
    id: 105_853_729,
    source_type: SourceType::Wikidata,
    name: "The Binding of Isaac: Rebirth animation",
    extensions: &["anm2"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x41, 0x6E, 0x69, 0x6D, 0x61, 0x74, 0x65, 0x64, 0x41, 0x63, 0x74, 0x6F,
                    0x72, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
