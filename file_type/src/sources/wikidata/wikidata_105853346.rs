use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853346: FileFormat = FileFormat {
    id: 105_853_346,
    source_type: SourceType::Wikidata,
    name: "WinAPE recorded session (full)",
    extensions: &["snr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x57, 0x20, 0x2D, 0x20, 0x53, 0x4E, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
