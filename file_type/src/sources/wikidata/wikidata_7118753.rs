use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7118753: FileFormat = FileFormat {
    id: 7_118_753,
    source_type: SourceType::Wikidata,
    name: "PDB",
    extensions: &["pdb"],
    media_types: &["application/vnd.palm"],
    signatures: &[],
    related_formats: &[],
};
