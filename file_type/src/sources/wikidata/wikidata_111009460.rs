use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009460: FileFormat = FileFormat {
    id: 111_009_460,
    puid: "wikidata/111009460",
    name: "PrintMaster Calendar File format",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
