use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863312: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_312,
        source_type: SourceType::Wikidata,
        name: "Mindjet MindManager Map",
        extensions: &["mmp"],
        media_types: &["application/vnd.mindjet.mindmanager"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
