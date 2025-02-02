use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852999: FileFormat = FileFormat {
    id: 105_852_999,
    source_type: SourceType::Wikidata,
    name: "GoatTracker chiptune (v2)",
    extensions: &["sng"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x54, 0x53, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
