use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663714: FileFormat = FileFormat {
    id: 66_663_714,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard Graphics Gallery",
    extensions: &["gal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
