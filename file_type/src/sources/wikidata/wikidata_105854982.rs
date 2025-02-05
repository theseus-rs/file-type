use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854982: FileFormat = FileFormat {
    id: 105_854_982,
    source_type: SourceType::Wikidata,
    name: "Binary caFe WAVe audio",
    extensions: &["bfwav"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x57, 0x41, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
