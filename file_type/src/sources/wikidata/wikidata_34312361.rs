use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34312361: FileFormat = FileFormat {
    id: 34_312_361,
    puid: "wikidata/34312361",
    name: "Macromedia Director, uncompressed Macintosh variant",
    extensions: &["dir", "dxr"],
    media_types: &["application/x-director", "application/x-director"],
    internal_signatures: &[],
    related_formats: &[],
};
