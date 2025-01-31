use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117230205: FileFormat = FileFormat {
    id: 117_230_205,
    puid: "wikidata/117230205",
    name: "PostScript file",
    extensions: &["ps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
