use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363609: FileFormat = FileFormat {
    id: 111_363_609,
    puid: "wikidata/111363609",
    name: "Westacott WinRanX instrument file",
    extensions: &["wrf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
