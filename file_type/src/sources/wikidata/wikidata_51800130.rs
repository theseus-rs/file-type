use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51800130: FileFormat = FileFormat {
    id: 51_800_130,
    puid: "wikidata/51800130",
    name: "Microsoft Excel Add-In",
    extensions: &["xla", "xll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
