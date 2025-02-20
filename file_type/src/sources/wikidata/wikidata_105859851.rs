use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859851: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_851,
        source_type: SourceType::Wikidata,
        name: "VP3 sewing machine file",
        extensions: &["vp3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x76, 0x73, 0x6D, 0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
