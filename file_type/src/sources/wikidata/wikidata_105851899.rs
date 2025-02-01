use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851899: FileFormat = FileFormat {
    id: 105_851_899,
    puid: "wikidata/105851899",
    name: "A'dam Music Composer Script",
    extensions: &["scr"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x65, 0x74, 0x20, 0x74, 0x65, 0x6D, 0x70, 0x6F, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
