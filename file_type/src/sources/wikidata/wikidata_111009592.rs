use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009592: FileFormat = FileFormat {
    id: 111_009_592,
    puid: "wikidata/111009592",
    name: "PrintMaster Label File format",
    extensions: &["lbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
