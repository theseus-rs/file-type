use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857140: FileFormat = FileFormat {
    id: 105_857_140,
    puid: "wikidata/105857140",
    name: "Descent game data archive",
    extensions: &["hog"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x48, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
