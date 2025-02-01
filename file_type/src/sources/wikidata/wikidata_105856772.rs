use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856772: FileFormat = FileFormat {
    id: 105_856_772,
    puid: "wikidata/105856772",
    name: "HxC Floppy Emulator firmware Update (generic)",
    extensions: &["upd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x58, 0x43, 0x46, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
