use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975834: FileFormat = FileFormat {
    id: 28_975_834,
    source_type: SourceType::Wikidata,
    name: "Tripos MOL2 molecule file",
    extensions: &["mol2"],
    media_types: &["chemical/x-mol2"],
    signatures: &[],
    related_formats: &[],
};
