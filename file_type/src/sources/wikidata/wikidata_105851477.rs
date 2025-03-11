use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851477: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_477,
        source_type: SourceType::Wikidata,
        name: "Trace32 Power Probe data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x74, 0x72, 0x61, 0x63, 0x65, 0x33, 0x32, 0x20, 0x70, 0x6F, 0x77, 0x65,
                        0x72, 0x20, 0x70, 0x72, 0x6F, 0x62, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
