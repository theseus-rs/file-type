use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857188: FileFormat = FileFormat {
    id: 105_857_188,
    source_type: SourceType::Wikidata,
    name: "Human Machine Interfaces MIDI Format",
    extensions: &["hmp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4D, 0x49, 0x4D, 0x49, 0x44, 0x49, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
