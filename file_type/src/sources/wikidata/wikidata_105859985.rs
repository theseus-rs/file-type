use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859985: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_985,
        source_type: SourceType::Wikidata,
        name: "PVA Video (MainAudioStream)",
        extensions: &["pva"],
        media_types: &["video/x-pva"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x56, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
