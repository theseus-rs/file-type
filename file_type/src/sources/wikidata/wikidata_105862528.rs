use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862528: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_528,
        source_type: SourceType::Wikidata,
        name: "Maker Interchange Format Book",
        extensions: &["mif"],
        media_types: &["application/vnd.mif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x42, 0x6F, 0x6F, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
