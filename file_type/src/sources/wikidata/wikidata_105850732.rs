use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_732,
        source_type: SourceType::Wikidata,
        name: "Java KeyStore",
        extensions: &["jks", "keystore"],
        media_types: &["application/x-java-keystore"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xED, 0xFE, 0xED])],
                },
            }],
        }],
        related_formats: &[],
    },
};
