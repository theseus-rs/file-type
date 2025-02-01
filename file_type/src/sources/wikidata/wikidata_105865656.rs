use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865656: FileFormat = FileFormat {
    id: 105_865_656,
    puid: "wikidata/105865656",
    name: "Garmin PCX5 track file",
    extensions: &["trk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x20, 0x20, 0x53, 0x4F, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x20, 0x4E,
                    0x41, 0x4D, 0x45, 0x20, 0x26, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
