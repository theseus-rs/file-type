use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852275: FileFormat = FileFormat {
    id: 105_852_275,
    puid: "wikidata/105852275",
    name: "RCA Studio 2 binary dump cartridge",
    extensions: &["st2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x43, 0x41, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
