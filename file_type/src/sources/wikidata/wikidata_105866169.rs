use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866169: FileFormat = FileFormat {
    id: 105_866_169,
    puid: "wikidata/105866169",
    name: "PlayStation Patch File (generic)",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x50, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
