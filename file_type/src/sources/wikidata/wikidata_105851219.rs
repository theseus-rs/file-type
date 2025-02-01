use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851219: FileFormat = FileFormat {
    id: 105_851_219,
    puid: "wikidata/105851219",
    name: "SlamDB Database (generic)",
    extensions: &["tdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x44, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
