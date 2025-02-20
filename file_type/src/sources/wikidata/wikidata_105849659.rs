use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849659: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_659,
        source_type: SourceType::Wikidata,
        name: "EGrid32 Compilable Grid Format (ready to be compiled)",
        extensions: &["cgf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x47, 0x33, 0x32, 0x43, 0x4D, 0x50, 0x5F, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
