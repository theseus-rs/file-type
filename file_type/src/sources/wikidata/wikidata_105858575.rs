use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858575: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_575,
        source_type: SourceType::Wikidata,
        name: "BeRoTracker module",
        extensions: &["brt"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x52, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
