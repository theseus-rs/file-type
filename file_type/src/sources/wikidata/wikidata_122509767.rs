use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122509767: FileFormat = FileFormat {
    id: 122_509_767,
    source_type: SourceType::Wikidata,
    name: "Pretty Good Privacy (PGP) Groups Data",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
