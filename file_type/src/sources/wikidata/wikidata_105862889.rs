use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862889: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_889,
        source_type: SourceType::Wikidata,
        name: "MacStitch/WinStitch Motif",
        extensions: &["motif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x4D, 0x6F, 0x74, 0x69,
                        0x66, 0x20, 0x56, 0x31, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
