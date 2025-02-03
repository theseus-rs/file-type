use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863406: FileFormat = FileFormat {
    id: 105_863_406,
    source_type: SourceType::Wikidata,
    name: "Fasttracker 2-channel Amiga Module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x43, 0x48, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
