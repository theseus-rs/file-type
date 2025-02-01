use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858857: FileFormat = FileFormat {
    id: 105_858_857,
    puid: "wikidata/105858857",
    name: "Roku Base Index Frames container",
    extensions: &["bif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x89, 0x42, 0x49, 0x46, 0x0D, 0x0A, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
