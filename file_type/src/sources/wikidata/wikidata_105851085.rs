use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851085: FileFormat = FileFormat {
    id: 105_851_085,
    puid: "wikidata/105851085",
    name: "Terragen project",
    extensions: &["tgd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x74, 0x65, 0x72, 0x72, 0x61, 0x67, 0x65, 0x6E, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
