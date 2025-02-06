use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860925: FileFormat = FileFormat {
    id: 105_860_925,
    source_type: SourceType::Wikidata,
    name: "Ruby RDoc Options",
    extensions: &["rdoc_options"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x2D, 0x20, 0x21, 0x72, 0x75, 0x62, 0x79, 0x2F, 0x6F, 0x62, 0x6A,
                    0x65, 0x63, 0x74, 0x3A, 0x52, 0x44, 0x6F, 0x63, 0x3A, 0x3A, 0x4F, 0x70, 0x74,
                    0x69, 0x6F, 0x6E, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
