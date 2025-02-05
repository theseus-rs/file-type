use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862657: FileFormat = FileFormat {
    id: 105_862_657,
    source_type: SourceType::Wikidata,
    name: "Fasttracker 6-channel Amiga Module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x36, 0x43, 0x48, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
