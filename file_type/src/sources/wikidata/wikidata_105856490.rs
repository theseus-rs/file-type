use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856490: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_490,
        source_type: SourceType::Wikidata,
        name: "Rigol waveform",
        extensions: &["wfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA5, 0xA5])],
                },
            }],
        }],
        related_formats: &[],
    },
};
