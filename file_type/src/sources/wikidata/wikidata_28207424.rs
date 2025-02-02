use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207424: FileFormat = FileFormat {
    id: 28_207_424,
    source_type: SourceType::Wikidata,
    name: "VEGX",
    extensions: &["egx", "vgx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
