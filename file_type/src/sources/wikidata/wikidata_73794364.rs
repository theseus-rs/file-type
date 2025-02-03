use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73794364: FileFormat = FileFormat {
    id: 73_794_364,
    source_type: SourceType::Wikidata,
    name: "QGIS Composer Template",
    extensions: &["qpt"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x73, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
