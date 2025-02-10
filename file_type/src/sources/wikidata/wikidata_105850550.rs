use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850550: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_550,
        source_type: SourceType::Wikidata,
        name: "ISO CDImage cue/description - Data (with remark)",
        extensions: &["cue"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x45, 0x4D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
