use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4746193: FileType = FileType {
    file_format: &FileFormat {
        id: 4_746_193,
        source_type: SourceType::Wikidata,
        name: "Amiga Disk File",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4F, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
