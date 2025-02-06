use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858233: FileFormat = FileFormat {
    id: 105_858_233,
    source_type: SourceType::Wikidata,
    name: "EXEPACK compressed DOS Executable",
    extensions: &["exe"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
