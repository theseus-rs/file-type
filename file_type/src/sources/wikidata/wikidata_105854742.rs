use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854742: FileFormat = FileFormat {
    id: 105_854_742,
    source_type: SourceType::Wikidata,
    name: "Team Developer / SQLWindows application (binary)",
    extensions: &["apl", "app"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x47, 0x44, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
