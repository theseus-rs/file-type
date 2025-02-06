use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860469: FileFormat = FileFormat {
    id: 105_860_469,
    source_type: SourceType::Wikidata,
    name: "GROMACS Residue Topology (with rem)",
    extensions: &["rtp"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
