use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867578: FileFormat = FileFormat {
    id: 105_867_578,
    source_type: SourceType::Wikidata,
    name: "NoteWorthy Composer song",
    extensions: &["nwc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4E, 0x57, 0x5A, 0x5D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
