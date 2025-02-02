use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48802090: FileFormat = FileFormat {
    id: 48_802_090,
    source_type: SourceType::Wikidata,
    name: "Aldus Freehand Drawing, version 3",
    extensions: &["fh3"],
    media_types: &["application/x-freehand"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x48, 0x33, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
