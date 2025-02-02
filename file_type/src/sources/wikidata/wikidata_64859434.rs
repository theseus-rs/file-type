use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64859434: FileFormat = FileFormat {
    id: 64_859_434,
    source_type: SourceType::Wikidata,
    name: "GEDCOM file format",
    extensions: &["ged"],
    media_types: &["text/vnd.familysearch.gedcom"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x20, 0x48, 0x45, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
