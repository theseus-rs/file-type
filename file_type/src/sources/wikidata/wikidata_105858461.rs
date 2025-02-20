use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858461: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_461,
        source_type: SourceType::Wikidata,
        name: "E-mu Emaxsynth sample",
        extensions: &["ez2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4D, 0x58, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
