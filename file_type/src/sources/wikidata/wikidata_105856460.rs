use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856460: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_460,
        source_type: SourceType::Wikidata,
        name: "Windows Imaging Format (generic)",
        extensions: &["esd", "swm", "wim"],
        media_types: &["application/x-ms-wim"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x57, 0x49, 0x4D, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
