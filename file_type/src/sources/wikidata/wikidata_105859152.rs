use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859152: FileFormat = FileFormat {
    id: 105_859_152,
    source_type: SourceType::Wikidata,
    name: "Platinen Layout Programm Bibliotheken/library",
    extensions: &["bib"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4C, 0x50, 0x00, 0x00, 0x82, 0x42, 0x49, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
