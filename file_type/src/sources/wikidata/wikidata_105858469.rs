use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858469: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_469,
        source_type: SourceType::Wikidata,
        name: "Ericsson eMelody Ringtone",
        extensions: &["emy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x45, 0x4D, 0x45, 0x4C, 0x4F, 0x44,
                        0x59, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
