use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_902,
        source_type: SourceType::Wikidata,
        name: "ANIMagic Map",
        extensions: &["map"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x50, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
