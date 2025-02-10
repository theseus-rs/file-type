use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851135: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_135,
        source_type: SourceType::Wikidata,
        name: "TRSI Sound Monitor song",
        extensions: &["tsm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x53, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
