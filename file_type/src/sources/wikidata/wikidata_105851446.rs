use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851446: FileFormat = FileFormat {
    id: 105_851_446,
    puid: "wikidata/105851446",
    name: "QuickTime Text subtitles",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x51, 0x54, 0x74, 0x65, 0x78, 0x74, 0x7D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
