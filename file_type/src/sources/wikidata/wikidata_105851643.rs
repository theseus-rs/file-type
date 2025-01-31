use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851643: FileFormat = FileFormat {
    id: 105_851_643,
    puid: "wikidata/105851643",
    name: "Stunt Island Take",
    extensions: &["tke"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x4C, 0x4D, 0x00, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
