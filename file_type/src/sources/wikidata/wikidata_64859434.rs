use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64859434: FileType = FileType {
    file_format: &FileFormat {
        id: 64_859_434,
        source_type: SourceType::Wikidata,
        name: "GEDCOM file format",
        extensions: &["ged"],
        media_types: &["text/vnd.familysearch.gedcom"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x20, 0x48, 0x45, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
