use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858873: FileFormat = FileFormat {
    id: 105_858_873,
    source_type: SourceType::Wikidata,
    name: "Calcomp raster bitmap",
    extensions: &["ccrf", "crf", "prn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
