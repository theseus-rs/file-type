use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852180: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_180,
        source_type: SourceType::Wikidata,
        name: "Ami Pro/Word Pro encrypted document",
        extensions: &["sam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x65, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
