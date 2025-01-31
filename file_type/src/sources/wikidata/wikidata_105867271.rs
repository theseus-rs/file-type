use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867271: FileFormat = FileFormat {
    id: 105_867_271,
    puid: "wikidata/105867271",
    name: "Eudora Address Book",
    extensions: &["nnt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x6C, 0x69, 0x61, 0x73, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
