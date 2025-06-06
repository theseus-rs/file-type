use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_308,
        source_type: SourceType::Wikidata,
        name: "git configuration",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x63, 0x6F, 0x72, 0x65, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
