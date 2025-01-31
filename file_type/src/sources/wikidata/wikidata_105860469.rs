use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860469: FileFormat = FileFormat {
    id: 105_860_469,
    puid: "wikidata/105860469",
    name: "GROMACS Residue Topology (with rem)",
    extensions: &["rtp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
