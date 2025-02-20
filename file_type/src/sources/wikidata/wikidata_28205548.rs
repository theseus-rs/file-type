use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205548: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_548,
        source_type: SourceType::Wikidata,
        name: "NewIcons",
        extensions: &["info"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE3, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
