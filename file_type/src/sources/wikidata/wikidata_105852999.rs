use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852999: FileFormat = FileFormat {
    id: 105_852_999,
    puid: "wikidata/105852999",
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
