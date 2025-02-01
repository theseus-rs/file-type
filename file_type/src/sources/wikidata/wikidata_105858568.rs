use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858568: FileFormat = FileFormat {
    id: 105_858_568,
    puid: "wikidata/105858568",
    name: "AMI BIOS logo/splash bitmap",
    extensions: &["grfx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x52, 0x46, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
