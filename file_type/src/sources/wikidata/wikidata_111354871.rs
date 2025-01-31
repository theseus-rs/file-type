use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111354871: FileFormat = FileFormat {
    id: 111_354_871,
    puid: "wikidata/111354871",
    name: "Steinberg LM-4 banks",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
