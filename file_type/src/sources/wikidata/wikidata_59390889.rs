use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59390889: FileFormat = FileFormat {
    id: 59_390_889,
    source_type: SourceType::Wikidata,
    name: "GraphPad Prism file format, version 4",
    extensions: &["pzf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
