use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857546: FileFormat = FileFormat {
    id: 105_857_546,
    source_type: SourceType::Wikidata,
    name: "IncrediMail Emoticon",
    extensions: &["ime"],
    media_types: &["application/vnd.ms-cab-compressed"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
