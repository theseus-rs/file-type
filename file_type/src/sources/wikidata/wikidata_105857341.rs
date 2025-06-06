use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857341: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_341,
        source_type: SourceType::Wikidata,
        name: "Java SunJCE KeyStore",
        extensions: &["jceks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCE, 0xCE, 0xCE, 0xCE, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
