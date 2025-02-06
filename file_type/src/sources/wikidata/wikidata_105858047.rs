use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858047: FileFormat = FileFormat {
    id: 105_858_047,
    source_type: SourceType::Wikidata,
    name: "Outlook 97 and 2000 E-mail Account Settings",
    extensions: &["iaf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x4D, 0x41, 0x49, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
