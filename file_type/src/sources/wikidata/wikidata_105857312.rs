use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857312: FileFormat = FileFormat {
    id: 105_857_312,
    source_type: SourceType::Wikidata,
    name: "Human Machine Interfaces MIDI Format (rev.unk)",
    extensions: &["hmi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4D, 0x49, 0x4D, 0x49, 0x44, 0x49, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
