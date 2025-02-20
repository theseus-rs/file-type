use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_959950: FileType = FileType {
    file_format: &FileFormat {
        id: 959_950,
        source_type: SourceType::Wikidata,
        name: "eXtensible Business Reporting Language",
        extensions: &["xbrl", "xml"],
        media_types: &["application/xml"],
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
