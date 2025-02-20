use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73515266: FileType = FileType {
    file_format: &FileFormat {
        id: 73_515_266,
        source_type: SourceType::Wikidata,
        name: "Protege Project",
        extensions: &["pprj"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
