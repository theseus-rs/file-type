use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851742: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_742,
        source_type: SourceType::Wikidata,
        name: "Aegis VideoSEG Script",
        extensions: &["script"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
