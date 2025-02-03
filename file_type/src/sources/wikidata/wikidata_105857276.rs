use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857276: FileFormat = FileFormat {
    id: 105_857_276,
    source_type: SourceType::Wikidata,
    name: "Borland Turbo Vision Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x42, 0x48, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
