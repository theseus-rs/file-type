use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856540: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_540,
        source_type: SourceType::Wikidata,
        name: "Altera Waveform Design File",
        extensions: &["wdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x44, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
