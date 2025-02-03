use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71264063: FileFormat = FileFormat {
    id: 71_264_063,
    source_type: SourceType::Wikidata,
    name: "Hauptwerk copy-protected samples format",
    extensions: &["hbw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0x48, 0x50, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
