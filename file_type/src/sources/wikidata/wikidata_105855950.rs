use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855950: FileFormat = FileFormat {
    id: 105_855_950,
    puid: "wikidata/105855950",
    name: "Project Dogwaffle animation (generic)",
    extensions: &["dwa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x57, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
