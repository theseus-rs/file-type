use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967202: FileFormat = FileFormat {
    id: 27_967_202,
    puid: "wikidata/27967202",
    name: "Nerdtracker II module",
    extensions: &["ned"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x45, 0x44, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
