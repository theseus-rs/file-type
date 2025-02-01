use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852127: FileFormat = FileFormat {
    id: 105_852_127,
    puid: "wikidata/105852127",
    name: "SubViewer 1.0 subtitles",
    extensions: &["sub"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
