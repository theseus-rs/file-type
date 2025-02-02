use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111512403: FileFormat = FileFormat {
    id: 111_512_403,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo .dat file (external)",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
