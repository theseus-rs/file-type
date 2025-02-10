use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859090: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_090,
        source_type: SourceType::Wikidata,
        name: "PrintFox/Pagefox bitmap (320x200)",
        extensions: &["bin", "bs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
