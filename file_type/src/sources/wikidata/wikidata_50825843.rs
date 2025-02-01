use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50825843: FileFormat = FileFormat {
    id: 50_825_843,
    puid: "wikidata/50825843",
    name: "AVCHD Index File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
