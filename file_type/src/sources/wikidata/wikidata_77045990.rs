use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_77045990: FileFormat = FileFormat {
    id: 77_045_990,
    source_type: SourceType::Wikidata,
    name: "Extensible 3D vector graphics (binary)",
    extensions: &["x3db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
