use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56291707: FileFormat = FileFormat {
    id: 56_291_707,
    puid: "wikidata/56291707",
    name: "Common Workflow Language",
    extensions: &["cwl", "cwl"],
    media_types: &["application/cwl", "application/cwl+json"],
    internal_signatures: &[],
    related_formats: &[],
};
