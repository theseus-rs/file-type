use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851166: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_166,
        source_type: SourceType::Wikidata,
        name: "Track Record Viewer TRV/TRVX definition",
        extensions: &["trv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x43, 0x46, 0x47, 0x29, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
