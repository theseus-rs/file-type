use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111512403: FileFormat = FileFormat {
    id: 111_512_403,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo .dat file (external)",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
