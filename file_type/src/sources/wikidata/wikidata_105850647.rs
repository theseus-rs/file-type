use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850647: FileFormat = FileFormat {
    id: 105_850_647,
    source_type: SourceType::Wikidata,
    name: "Koda Form Designer Form",
    extensions: &["kxf"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
