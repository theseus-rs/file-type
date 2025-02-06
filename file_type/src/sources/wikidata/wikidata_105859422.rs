use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859422: FileFormat = FileFormat {
    id: 105_859_422,
    source_type: SourceType::Wikidata,
    name: "Quartus Waveform simulation",
    extensions: &["vwf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
