use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850170: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_170,
        source_type: SourceType::Wikidata,
        name: "Cheat Engine Cheat Table",
        extensions: &["ct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x45, 0x41, 0x54, 0x45, 0x4E, 0x47, 0x49, 0x4E, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
