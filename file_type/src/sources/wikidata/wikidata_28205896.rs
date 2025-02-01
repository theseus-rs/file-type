use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205896: FileFormat = FileFormat {
    id: 28_205_896,
    puid: "wikidata/28205896",
    name: "DESR VFF",
    extensions: &["vff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
