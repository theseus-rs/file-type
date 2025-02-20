use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854221: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_221,
        source_type: SourceType::Wikidata,
        name: "PS/2 MicroChannel Adapter Description File (with CRLF)",
        extensions: &["adf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
