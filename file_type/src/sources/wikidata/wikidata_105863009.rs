use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863009: FileFormat = FileFormat {
    id: 105_863_009,
    puid: "wikidata/105863009",
    name: "StoneTracker Module",
    extensions: &["spm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x4D, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
