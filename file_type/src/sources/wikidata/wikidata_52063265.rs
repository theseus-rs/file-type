use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063265: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_265,
        source_type: SourceType::Wikidata,
        name: "Lotus WordPro Document",
        extensions: &["lwp"],
        media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x6F, 0x72, 0x64, 0x50, 0x72, 0x6F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
