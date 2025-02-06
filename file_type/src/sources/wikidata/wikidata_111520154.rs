use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111520154: FileFormat = FileFormat {
    id: 111_520_154,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo DAT File (internal)",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
