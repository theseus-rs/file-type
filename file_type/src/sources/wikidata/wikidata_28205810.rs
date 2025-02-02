use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205810: FileFormat = FileFormat {
    id: 28_205_810,
    source_type: SourceType::Wikidata,
    name: "Calamus Raster Graphic",
    extensions: &["crg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x43, 0x52, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
