use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919155: FileFormat = FileFormat {
    id: 28_919_155,
    source_type: SourceType::Wikidata,
    name: "Rhino Worksession",
    extensions: &["rws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
