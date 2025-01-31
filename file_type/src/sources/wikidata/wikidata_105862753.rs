use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862753: FileFormat = FileFormat {
    id: 105_862_753,
    puid: "wikidata/105862753",
    name: "TMPEGEnc config file",
    extensions: &["mcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x54, 0x4D, 0x50, 0x45, 0x47, 0x43,
                    0x6F, 0x6E, 0x66, 0x69, 0x67, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
