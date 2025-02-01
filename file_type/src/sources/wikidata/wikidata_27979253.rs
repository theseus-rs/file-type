use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979253: FileFormat = FileFormat {
    id: 27_979_253,
    puid: "wikidata/27979253",
    name: "PCBoard",
    extensions: &["pcb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
