use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975834: FileFormat = FileFormat {
    id: 28_975_834,
    puid: "wikidata/28975834",
    name: "Tripos MOL2 molecule file",
    extensions: &["mol2"],
    media_types: &["chemical/x-mol2"],
    internal_signatures: &[],
    related_formats: &[],
};
