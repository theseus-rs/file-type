use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600246: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_246,
        source_type: SourceType::Wikidata,
        name: "FreeArc ARC",
        extensions: &["arc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x72, 0x43, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
