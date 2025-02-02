use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_673906: FileFormat = FileFormat {
    id: 673_906,
    source_type: SourceType::Wikidata,
    name: "Simple Standards-Based Slide Show System",
    extensions: &["html", "xhtml"],
    media_types: &["application/xhtml+xml", "text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
