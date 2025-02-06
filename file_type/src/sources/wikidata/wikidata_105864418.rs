use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864418: FileFormat = FileFormat {
    id: 105_864_418,
    source_type: SourceType::Wikidata,
    name: "Commodore 128 BASIC V7.0 program (graph mode on)",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
