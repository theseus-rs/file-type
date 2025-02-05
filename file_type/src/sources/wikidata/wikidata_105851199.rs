use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851199: FileFormat = FileFormat {
    id: 105_851_199,
    source_type: SourceType::Wikidata,
    name: "3 Step Studio Project",
    extensions: &["tssproj"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
