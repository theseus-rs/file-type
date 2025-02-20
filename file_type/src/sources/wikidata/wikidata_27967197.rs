use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967197: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_197,
        source_type: SourceType::Wikidata,
        name: "KRIS Packer/ChipTracker module",
        extensions: &["kris"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x52, 0x49, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
