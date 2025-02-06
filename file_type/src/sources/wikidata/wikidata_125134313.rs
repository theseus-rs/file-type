use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125134313: FileFormat = FileFormat {
    id: 125_134_313,
    source_type: SourceType::Wikidata,
    name: "YAM emailcache",
    extensions: &["emailcache"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
