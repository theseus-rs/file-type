use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120717288: FileFormat = FileFormat {
    id: 120_717_288,
    puid: "wikidata/120717288",
    name: "TaxCut 2007 Tax Return file",
    extensions: &["t07"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
