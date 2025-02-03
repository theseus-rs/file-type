use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_68480634: FileFormat = FileFormat {
    id: 68_480_634,
    source_type: SourceType::Wikidata,
    name: "Magic: The Gathering cards Deck file format",
    extensions: &["dec"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x4E, 0x41, 0x4D, 0x45, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
