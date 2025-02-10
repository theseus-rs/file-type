use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_769,
        source_type: SourceType::Wikidata,
        name: "Adventure SOS compiled walkthrough",
        extensions: &["sos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x41, 0x64, 0x53, 0x4F, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
