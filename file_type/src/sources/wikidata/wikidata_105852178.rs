use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852178: FileFormat = FileFormat {
    id: 105_852_178,
    puid: "wikidata/105852178",
    name: "Spectrum Sound Tracker 1.1 chiptune",
    extensions: &["st11"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x41, 0x59, 0x53, 0x54, 0x31, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
