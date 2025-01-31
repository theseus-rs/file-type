use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79238872: FileFormat = FileFormat {
    id: 79_238_872,
    puid: "wikidata/79238872",
    name: "Palm Address Book",
    extensions: &["aba"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x42, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
