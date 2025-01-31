use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63415958: FileFormat = FileFormat {
    id: 63_415_958,
    puid: "wikidata/63415958",
    name: "Graphic Workshop for Windows Thumbnail File",
    extensions: &["thn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x4E, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
