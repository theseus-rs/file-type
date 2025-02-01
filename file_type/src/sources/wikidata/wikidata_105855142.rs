use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855142: FileFormat = FileFormat {
    id: 105_855_142,
    puid: "wikidata/105855142",
    name: "HxC Floppy Emulator Floppy Profile (v0.1)",
    extensions: &["fpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x50, 0x46, 0x5F, 0x56, 0x30, 0x2E, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
