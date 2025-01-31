use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205901: FileFormat = FileFormat {
    id: 28_205_901,
    puid: "wikidata/28205901",
    name: "DGI",
    extensions: &["dgi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
