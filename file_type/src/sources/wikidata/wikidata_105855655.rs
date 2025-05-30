use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_655,
        source_type: SourceType::Wikidata,
        name: "Cakewalk Overture Score",
        extensions: &["ove"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x56, 0x53, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
