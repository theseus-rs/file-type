use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857552: FileFormat = FileFormat {
    id: 105_857_552,
    puid: "wikidata/105857552",
    name: "EZ-DiskCopy PRO disk image",
    extensions: &["rim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x5A, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
