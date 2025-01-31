use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111520154: FileFormat = FileFormat {
    id: 111_520_154,
    puid: "wikidata/111520154",
    name: "ESRI ArcInfo DAT File (internal)",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
