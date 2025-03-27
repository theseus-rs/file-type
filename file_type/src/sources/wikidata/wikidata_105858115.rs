use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858115: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_115,
        source_type: SourceType::Wikidata,
        name: "International Patching System binary patch",
        extensions: &["ips"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x54, 0x43, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
