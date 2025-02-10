use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_875,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for DOS Style sheet",
        extensions: &["stx", "sty"],
        media_types: &["application/msword", "application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
