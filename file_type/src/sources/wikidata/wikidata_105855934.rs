use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855934: FileFormat = FileFormat {
    id: 105_855_934,
    source_type: SourceType::Wikidata,
    name: "MSX-DOS disk Image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEB, 0xFE, 0x90])],
            },
        }],
    }],
    related_formats: &[],
};
