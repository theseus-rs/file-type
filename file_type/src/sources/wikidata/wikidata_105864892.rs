use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864892: FileFormat = FileFormat {
    id: 105_864_892,
    puid: "wikidata/105864892",
    name: "Piping Component File",
    extensions: &["pcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x4F, 0x47, 0x45, 0x4E, 0x2D, 0x46, 0x49, 0x4C, 0x45, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
