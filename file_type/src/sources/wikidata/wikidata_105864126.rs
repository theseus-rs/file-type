use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864126: FileFormat = FileFormat {
    id: 105_864_126,
    source_type: SourceType::Wikidata,
    name: "Octalyser 8-channel STe/Falcon Module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x38, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
