use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857184: FileFormat = FileFormat {
    id: 105_857_184,
    source_type: SourceType::Wikidata,
    name: "Amiga Rigid Disk block / Hard Disk File image",
    extensions: &["hdf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x53, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
