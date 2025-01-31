use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852902: FileFormat = FileFormat {
    id: 105_852_902,
    puid: "wikidata/105852902",
    name: "GoatTracker chiptune",
    extensions: &["sng"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x54, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
