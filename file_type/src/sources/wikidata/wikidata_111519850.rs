use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111519850: FileFormat = FileFormat {
    id: 111_519_850,
    puid: "wikidata/111519850",
    name: "Stata .do command file",
    extensions: &["do", "do", "do"],
    media_types: &["application/x-stata", "text/stata", "text/x-stata"],
    internal_signatures: &[],
    related_formats: &[],
};
