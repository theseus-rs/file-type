use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853795: FileFormat = FileFormat {
    id: 105_853_795,
    puid: "wikidata/105853795",
    name: "IRCAM Sound Format audio (MIPS DECstation)",
    extensions: &["sf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0xA3, 0x03, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
