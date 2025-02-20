use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_663,
        source_type: SourceType::Wikidata,
        name: "Wii Model Animation",
        extensions: &["chr0"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x52, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
