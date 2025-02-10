use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_977900: FileFormat = FileFormat {
    id: 977_900,
    source_type: SourceType::Wikidata,
    name: "Open XML Paper Specification",
    extensions: &["oxps", "xps"],
    media_types: &["application/oxps", "application/vnd.ms-xpsdocument"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
