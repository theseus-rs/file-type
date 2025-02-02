use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110212801: FileFormat = FileFormat {
    id: 110_212_801,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X5",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
