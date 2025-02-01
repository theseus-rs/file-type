use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856902: FileFormat = FileFormat {
    id: 105_856_902,
    puid: "wikidata/105856902",
    name: "LucasArts Game data archive (generic)",
    extensions: &["gob"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
