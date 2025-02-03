use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111520154: FileFormat = FileFormat {
    id: 111_520_154,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo DAT File (internal)",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
