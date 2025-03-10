use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762721: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_721,
        source_type: SourceType::Wikidata,
        name: "News Industry Text Format",
        extensions: &[],
        media_types: &["text/vnd.iptc.nitf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
