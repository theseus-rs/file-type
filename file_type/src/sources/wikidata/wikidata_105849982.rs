use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849982: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_982,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal 3.0 Chain module",
        extensions: &["chn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE8, 0xFE, 0xDD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
