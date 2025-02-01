use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131413711: FileFormat = FileFormat {
    id: 131_413_711,
    puid: "wikidata/131413711",
    name: "VisualProlog grammar file format",
    extensions: &["vipgrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
