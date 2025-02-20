use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67175428: FileType = FileType {
    file_format: &FileFormat {
        id: 67_175_428,
        source_type: SourceType::Wikidata,
        name: "StarView Metafile",
        extensions: &["svm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x43, 0x4C, 0x4D, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
