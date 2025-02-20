use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863613: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_613,
        source_type: SourceType::Wikidata,
        name: "Max Payne data file",
        extensions: &["ras"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x41, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
