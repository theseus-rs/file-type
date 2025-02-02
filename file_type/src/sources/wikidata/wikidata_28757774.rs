use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757774: FileFormat = FileFormat {
    id: 28_757_774,
    source_type: SourceType::Wikidata,
    name: "GEXF",
    extensions: &["gexf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
