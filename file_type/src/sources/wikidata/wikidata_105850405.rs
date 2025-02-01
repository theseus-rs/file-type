use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850405: FileFormat = FileFormat {
    id: 105_850_405,
    puid: "wikidata/105850405",
    name: "SuperJAM! Chords",
    extensions: &["chords"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x52, 0x44, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
