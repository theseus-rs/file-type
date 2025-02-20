use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25110402: FileType = FileType {
    file_format: &FileFormat {
        id: 25_110_402,
        source_type: SourceType::Wikidata,
        name: "Personal Filing Cabinet",
        extensions: &["pfc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4F, 0x4C, 0x56, 0x4D, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
