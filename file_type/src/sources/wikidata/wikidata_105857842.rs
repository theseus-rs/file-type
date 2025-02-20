use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857842: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_842,
        source_type: SourceType::Wikidata,
        name: "Psion Organiser Pack image",
        extensions: &["opk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
