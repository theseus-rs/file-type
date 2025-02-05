use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863196: FileFormat = FileFormat {
    id: 105_863_196,
    source_type: SourceType::Wikidata,
    name: "TCB Tracker module",
    extensions: &["tcb"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
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
