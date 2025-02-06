use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851248: FileFormat = FileFormat {
    id: 105_851_248,
    source_type: SourceType::Wikidata,
    name: "Better Working Eight-In-One spreadsheet",
    extensions: &["tpl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x45, 0x43, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
