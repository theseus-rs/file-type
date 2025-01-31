use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867486: FileFormat = FileFormat {
    id: 105_867_486,
    puid: "wikidata/105867486",
    name: "Mario Kart DS track data",
    extensions: &["nkm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x4B, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
