use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117287754: FileFormat = FileFormat {
    id: 117_287_754,
    puid: "wikidata/117287754",
    name: "SigmaPlot Notebook File",
    extensions: &["jnb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
