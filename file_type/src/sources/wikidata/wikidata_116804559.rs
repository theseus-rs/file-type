use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116804559: FileFormat = FileFormat {
    id: 116_804_559,
    puid: "wikidata/116804559",
    name: "TimeWiz Project File",
    extensions: &["twz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
