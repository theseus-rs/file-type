use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117338260: FileFormat = FileFormat {
    id: 117_338_260,
    puid: "wikidata/117338260",
    name: "Corel Library",
    extensions: &["clb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
