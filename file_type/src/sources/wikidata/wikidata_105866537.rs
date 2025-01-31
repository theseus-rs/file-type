use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866537: FileFormat = FileFormat {
    id: 105_866_537,
    puid: "wikidata/105866537",
    name: "Korg Trinity/Triton instruments bank (generic)",
    extensions: &["pcg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x4F, 0x52, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
