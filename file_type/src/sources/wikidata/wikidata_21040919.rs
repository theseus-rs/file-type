use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21040919: FileFormat = FileFormat {
    id: 21_040_919,
    puid: "wikidata/21040919",
    name: "MultiTracker format",
    extensions: &["mtm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x54, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
