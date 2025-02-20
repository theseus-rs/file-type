use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853337: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_337,
        source_type: SourceType::Wikidata,
        name: "VirtuaNES savestate",
        extensions: &["st0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x4E, 0x45, 0x53, 0x20, 0x53, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
