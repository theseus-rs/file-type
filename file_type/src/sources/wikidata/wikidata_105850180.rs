use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850180: FileFormat = FileFormat {
    id: 105_850_180,
    source_type: SourceType::Wikidata,
    name: "CyberTracker 64 chiptune",
    extensions: &["ct"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x04, 0x4E, 0x4E, 0x54, 0x52, 0x4B, 0x4D, 0x5A, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
