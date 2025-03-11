use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866111: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_111,
        source_type: SourceType::Wikidata,
        name: "PFS Professional Planner spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x00, 0x0C, 0x00, 0x44, 0x6F, 0x6C, 0x70, 0x68, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
