use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851111: FileFormat = FileFormat {
    id: 105_851_111,
    puid: "wikidata/105851111",
    name: "Tree Generator 3D tree",
    extensions: &["tgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x47, 0x46, 0x20, 0x62, 0x79, 0x20, 0x54, 0x72, 0x65, 0x65, 0x20, 0x47,
                    0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
