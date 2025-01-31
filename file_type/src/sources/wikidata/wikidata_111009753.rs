use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009753: FileFormat = FileFormat {
    id: 111_009_753,
    puid: "wikidata/111009753",
    name: "PrintMaster Web Page File format",
    extensions: &["web"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
