use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_743275: FileFormat = FileFormat {
    id: 743_275,
    source_type: SourceType::Wikidata,
    name: "T.38",
    extensions: &["t38"],
    media_types: &["audio/t38", "image/t38"],
    internal_signatures: &[],
    related_formats: &[],
};
