use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125134313: FileFormat = FileFormat {
    id: 125_134_313,
    source_type: SourceType::Wikidata,
    name: "YAM emailcache",
    extensions: &["emailcache"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
