use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_893,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Chart (vA.01)",
        extensions: &["cht"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x4F, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
