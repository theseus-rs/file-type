use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856419: FileFormat = FileFormat {
    id: 105_856_419,
    source_type: SourceType::Wikidata,
    name: "Tektronix TDS waveform data",
    extensions: &["wfm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4C, 0x57, 0x46, 0x4D, 0x20, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
