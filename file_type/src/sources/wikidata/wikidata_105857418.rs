use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857418: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_418,
        source_type: SourceType::Wikidata,
        name: "Ways Job Control",
        extensions: &["joc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x6F, 0x62, 0x57, 0x69, 0x6E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
