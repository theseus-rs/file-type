use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_256: FileType = FileType {
    file_format: &FileFormat {
        id: 256,
        source_type: SourceType::Pronom,
        name: "RealAudio Metafile",
        extensions: &["ram"],
        media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x74, 0x73, 0x70, 0x3A, 0x2F, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
