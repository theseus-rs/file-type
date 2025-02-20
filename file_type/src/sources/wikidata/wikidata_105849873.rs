use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_873,
        source_type: SourceType::Wikidata,
        name: "ISO CDImage cue/description - Data",
        extensions: &["cue"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x49, 0x4C, 0x45, 0x20, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
