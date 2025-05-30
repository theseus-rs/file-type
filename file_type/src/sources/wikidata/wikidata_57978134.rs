use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_57978134: FileType = FileType {
    file_format: &FileFormat {
        id: 57_978_134,
        source_type: SourceType::Wikidata,
        name: "ASP Control Directive File",
        extensions: &["ascx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x25, 0x40, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
