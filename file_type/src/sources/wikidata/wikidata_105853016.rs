use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853016: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_016,
        source_type: SourceType::Wikidata,
        name: "SkyOS Installation File",
        extensions: &["sif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x47, 0x45, 0x4E, 0x45, 0x52, 0x41, 0x4C, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
