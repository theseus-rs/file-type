use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858150: FileFormat = FileFormat {
    id: 105_858_150,
    puid: "wikidata/105858150",
    name: "Cura profile settings (legacy)",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x70, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
