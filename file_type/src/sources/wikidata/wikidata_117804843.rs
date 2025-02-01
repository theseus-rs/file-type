use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117804843: FileFormat = FileFormat {
    id: 117_804_843,
    puid: "wikidata/117804843",
    name: "Working Model 2D File",
    extensions: &["wm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
