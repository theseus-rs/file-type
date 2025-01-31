use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857613: FileFormat = FileFormat {
    id: 105_857_613,
    puid: "wikidata/105857613",
    name: "Fink Info-file",
    extensions: &["info"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
