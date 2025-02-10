use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857312: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_312,
        source_type: SourceType::Wikidata,
        name: "Human Machine Interfaces MIDI Format (rev.unk)",
        extensions: &["hmi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
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
    },
};
