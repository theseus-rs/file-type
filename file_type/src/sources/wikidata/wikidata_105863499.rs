use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863499: FileFormat = FileFormat {
    id: 105_863_499,
    source_type: SourceType::Wikidata,
    name: "MusicMatch JukeBox Visualization (v1.0)",
    extensions: &["mvs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4D, 0x56, 0x53, 0x31, 0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
