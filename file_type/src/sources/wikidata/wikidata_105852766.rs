use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852766: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_766,
        source_type: SourceType::Wikidata,
        name: "SNSF Super Nintendo Sound Format rip",
        extensions: &["snsf", "snsflib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x46, 0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
