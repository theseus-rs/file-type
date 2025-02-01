use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851437: FileFormat = FileFormat {
    id: 105_851_437,
    puid: "wikidata/105851437",
    name: "Textra Writer (generic)",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0xFD, 0xFF, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
