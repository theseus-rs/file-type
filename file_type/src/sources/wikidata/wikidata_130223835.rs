use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130223835: FileFormat = FileFormat {
    id: 130_223_835,
    puid: "wikidata/130223835",
    name: "Lean 3 file format",
    extensions: &["lean", "lean"],
    media_types: &["text/x-lean", "text/x-lean3"],
    internal_signatures: &[],
    related_formats: &[],
};
