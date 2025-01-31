use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858143: FileFormat = FileFormat {
    id: 105_858_143,
    puid: "wikidata/105858143",
    name: "PGP Disk image",
    extensions: &["pgd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x47, 0x50, 0x64, 0x4D, 0x41, 0x49, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
