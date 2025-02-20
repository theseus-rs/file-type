use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850594: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_594,
        source_type: SourceType::Wikidata,
        name: "Minimig Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4E, 0x4D, 0x47, 0x43, 0x46, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
