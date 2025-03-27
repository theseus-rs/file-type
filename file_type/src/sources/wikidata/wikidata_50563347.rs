use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50563347: FileType = FileType {
    file_format: &FileFormat {
        id: 50_563_347,
        source_type: SourceType::Wikidata,
        name: "Adobe Audio Waveform",
        extensions: &["pek"],
        media_types: &["application/x-pek"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x54, 0x23, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
