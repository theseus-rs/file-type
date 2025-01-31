use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864290: FileFormat = FileFormat {
    id: 105_864_290,
    puid: "wikidata/105864290",
    name: "Novalogic game data archive (PFF3)",
    extensions: &["pff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x46, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
