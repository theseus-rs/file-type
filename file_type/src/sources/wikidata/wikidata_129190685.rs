use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129190685: FileFormat = FileFormat {
    id: 129_190_685,
    puid: "wikidata/129190685",
    name: "FStar file format",
    extensions: &["fst"],
    media_types: &["text/x-fstar"],
    internal_signatures: &[],
    related_formats: &[],
};
