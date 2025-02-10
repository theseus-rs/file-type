use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867194: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_194,
        source_type: SourceType::Wikidata,
        name: "NovoTrade Packer module",
        extensions: &["ntp"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x44, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
