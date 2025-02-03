use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72825855: FileFormat = FileFormat {
    id: 72_825_855,
    source_type: SourceType::Wikidata,
    name: "OpenCanvas Image",
    extensions: &["oci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
