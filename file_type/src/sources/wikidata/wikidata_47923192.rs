use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47923192: FileFormat = FileFormat {
    id: 47_923_192,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Macintosh Document, version 5.0",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x37, 0x00, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
