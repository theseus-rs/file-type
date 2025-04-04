use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851502: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_502,
        source_type: SourceType::Wikidata,
        name: "Atari ST TOS executable",
        extensions: &["tos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x60, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
