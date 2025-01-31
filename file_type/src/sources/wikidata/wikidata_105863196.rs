use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863196: FileFormat = FileFormat {
    id: 105_863_196,
    puid: "wikidata/105863196",
    name: "TCB Tracker module",
    extensions: &["tcb"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4E, 0x20, 0x43, 0x4F, 0x4F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
