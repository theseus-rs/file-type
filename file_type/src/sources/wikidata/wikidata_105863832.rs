use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863832: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_832,
        source_type: SourceType::Wikidata,
        name: "Quartus Memory Initialization File",
        extensions: &["mif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
