use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049603: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_603,
        source_type: SourceType::Wikidata,
        name: "Tiny Stuff, medium resolution",
        extensions: &["tn2", "tny"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x07, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
