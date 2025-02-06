use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853333: FileFormat = FileFormat {
    id: 105_853_333,
    source_type: SourceType::Wikidata,
    name: "Vim swap",
    extensions: &["swp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x30, 0x56, 0x49, 0x4D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
