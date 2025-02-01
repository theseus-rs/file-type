use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850145: FileFormat = FileFormat {
    id: 105_850_145,
    puid: "wikidata/105850145",
    name: "Black and White 2 game data script",
    extensions: &["chl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x48, 0x56, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
