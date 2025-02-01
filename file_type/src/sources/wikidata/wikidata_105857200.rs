use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857200: FileFormat = FileFormat {
    id: 105_857_200,
    puid: "wikidata/105857200",
    name: "Hively Tracker module",
    extensions: &["hvl"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x56, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
