use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854603: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_603,
        source_type: SourceType::Wikidata,
        name: "Lavasoft Ad-Aware reference file",
        extensions: &["ref"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x1B, 0x53, 0x54, 0x44, 0x50, 0x53, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
