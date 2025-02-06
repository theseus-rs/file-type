use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_743275: FileFormat = FileFormat {
    id: 743_275,
    source_type: SourceType::Wikidata,
    name: "T.38",
    extensions: &["t38"],
    media_types: &["audio/t38", "image/t38"],
    signatures: &[],
    related_formats: &[],
};
