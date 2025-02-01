use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167848: FileFormat = FileFormat {
    id: 29_167_848,
    puid: "wikidata/29167848",
    name: "Outlook Express Database",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCF, 0xAD, 0x12, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
