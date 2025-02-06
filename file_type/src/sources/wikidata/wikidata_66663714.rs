use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663714: FileFormat = FileFormat {
    id: 66_663_714,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard Graphics Gallery",
    extensions: &["gal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
