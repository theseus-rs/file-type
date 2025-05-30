use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855785: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_785,
        source_type: SourceType::Wikidata,
        name: "Mini Office text Document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x0D, 0x80, 0xCF, 0x80, 0x81])],
                },
            }],
        }],
        related_formats: &[],
    },
};
