use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866006: FileFormat = FileFormat {
    id: 105_866_006,
    puid: "wikidata/105866006",
    name: "Crouzet Logic Software M3 project",
    extensions: &["pm3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x52, 0x4F, 0x55, 0x5A, 0x45, 0x54, 0x5F, 0x4D, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
