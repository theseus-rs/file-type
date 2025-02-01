use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967148: FileFormat = FileFormat {
    id: 27_967_148,
    puid: "wikidata/27967148",
    name: "Extreme's Tracker module",
    extensions: &["ams"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x78, 0x74, 0x72, 0x65, 0x6D, 0x65, 0x30, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
