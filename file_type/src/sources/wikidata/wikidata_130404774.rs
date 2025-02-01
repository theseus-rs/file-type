use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130404774: FileFormat = FileFormat {
    id: 130_404_774,
    puid: "wikidata/130404774",
    name: "OpenEdge ABL source code file",
    extensions: &["p", "p"],
    media_types: &["application/x-openedge", "text/x-openedge"],
    internal_signatures: &[],
    related_formats: &[],
};
