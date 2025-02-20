use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864781: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_781,
        source_type: SourceType::Wikidata,
        name: "Creative C/MS packed screen",
        extensions: &["hpc", "mpc", "pic", "tpc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x61, 0x63, 0x6B, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
